/* Date Created: 02/07/2025. */

//! The **A (Not Very) Pretty Printer** in  
//! [https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer](https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer)
//! 
//! The full original Java version can be found at:
//! https://github.com/munificent/craftinginterpreters/blob/master/java/com/craftinginterpreters/lox/AstPrinter.java

// To run test for this module only: 
// 
//     * cargo test ast_printer::tests

use std::rc::Rc;

use super::lox_error::LoxError;
use super::lox_runtime_error::LoxRuntimeError;
use super::token::{LiteralValue, Token};

use super::expr;
use super::stmt;
use super::{unwrap_expr, unwrap_stmt};

// Rust-specific.
pub enum AstFragment<'a> {
    Expr(Rc<expr::Expr>),
    Stmt(Rc<stmt::Stmt>),
    Token(&'a Token),
    Text(String),
    Group(Vec<AstFragment<'a>>),
}

impl<'a> AstFragment<'a> {
    fn write_to(&self, printer: &mut AstPrinter, builder: &mut String) {
        builder.push(' ');
        match self {
            AstFragment::Expr(e) => builder.push_str(
                &expr::Expr::accept_ref(e.clone(), printer)
                    .unwrap_or_else(|err| format!("[Error printing expression: {}]", err))
            ),
            AstFragment::Stmt(s) => builder.push_str(
                &stmt::Stmt::accept_ref(Rc::clone(s), printer)
                    .unwrap_or_else(|err| format!("[Error printing statement: {}]", err))
            ),
            AstFragment::Token(t) => builder.push_str(t.lexeme()),
            AstFragment::Text(s) => builder.push_str(s),
            AstFragment::Group(g) => {
                for sub in g {
                    sub.write_to(printer, builder);
                }
            }
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print_expression(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        expr::Expr::accept_ref(expr, self)
    }

    pub fn print_statement(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        stmt::Stmt::accept_ref(stmt, self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[Rc<expr::Expr>]) -> Result<String, LoxError> {
        let mut builder = String::from(format!("({}", name));

        for e in exprs {
            builder.push_str(&format!(" {}", expr::Expr::accept_ref(e.clone(), self)?));
        }        
        builder.push_str(")");

        Ok(builder)
    }

    fn transform(&mut self, builder: &mut String, parts: &[AstFragment]) {
        for part in parts {
            builder.push(' ');
            match part {
                AstFragment::Expr(e) => builder.push_str(
                    &expr::Expr::accept_ref(e.clone(), self)
                        .unwrap_or_else(|err| format!("[Error printing expression: {}]", err))
                ),                
                AstFragment::Stmt(stmt) => builder.push_str(
                    &stmt::Stmt::accept_ref(Rc::clone(stmt), self)
                        .unwrap_or_else(|err| format!("[Error printing statement: {}]", err))
                ),
                AstFragment::Token(token) => builder.push_str(token.lexeme()),
                AstFragment::Text(s) => builder.push_str(s),
                AstFragment::Group(subparts) => self.transform(builder, subparts),
            }
        }
    }  

    // Note by the author, Mr. Robert Nystrom:
    // Note: AstPrinting other types of syntax trees is not shown in the
    // book, but this is provided here as a reference for those reading
    // the full code.    
    fn parenthesize2(&mut self, name: &str, parts: &[AstFragment]) -> Result<String, LoxError> {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        self.transform(&mut builder, parts);
        builder.push(')');
        Ok(builder)
    }

}

impl expr::Visitor<String> for AstPrinter {
    // My note: untested.
    fn visit_assign_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let assign = unwrap_expr!(expr, Assign);

        Ok(self.parenthesize2(
            &assign.name().lexeme(),
            &[
                AstFragment::Expr(Rc::clone(assign.value())),
            ],
        )?)
    }

    fn visit_binary_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let binary = unwrap_expr!(expr, Binary);

        Ok(self.parenthesize(binary.operator().lexeme(), 
        &[Rc::clone(binary.left()), Rc::clone(binary.right())])?)
    }    

    // My note: untested.
    fn visit_call_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let call = unwrap_expr!(expr, Call);

        let mut fragments = vec![AstFragment::Expr(Rc::clone(call.callee()))];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            call.arguments()
                .iter()
                .map(|arg| AstFragment::Expr(arg.clone()))
        );

        Ok(self.parenthesize2("call", &fragments)?)
    }

    // My note: untested.
    fn visit_get_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let get = unwrap_expr!(expr, Get);

        Ok(self.parenthesize2(
            ".",
            &[
                AstFragment::Expr(Rc::clone(get.object())),
                AstFragment::Text(get.name().lexeme().to_string()),
            ],
        )?)
    }
    
    fn visit_grouping_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let grouping = unwrap_expr!(expr, Grouping);

        Ok(self.parenthesize("group", &[Rc::clone(grouping.expression())])?)
    }

    fn visit_literal_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let literal = unwrap_expr!(expr, Literal);

        match literal.value() {
            LiteralValue::Number(n) => Ok(format!("{:?}", n)),
            LiteralValue::String(s) => Ok(s.to_string()),
            LiteralValue::Boolean(b) => Ok(b.to_string()),
            LiteralValue::Nil => Ok("nil".to_string()),
        }        
    }

    // My note: untested.
    fn visit_logical_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let logical = unwrap_expr!(expr, Logical);

        Ok(self.parenthesize(
            logical.operator().lexeme(), 
            &[
                        Rc::clone(logical.left()),
                        Rc::clone(logical.right()),
                    ]
        )?)
    }

    // My note: untested.
    fn visit_set_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let set = unwrap_expr!(expr, Set);

        Ok(self.parenthesize2(
            "=",
            &[
                AstFragment::Expr(Rc::clone(set.object())),
                AstFragment::Text(set.name().lexeme().to_string()),
                AstFragment::Expr(Rc::clone(set.value())),
            ],
        )?)
    }

    // My note: untested.
    fn visit_super_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let inner = unwrap_expr!(expr, Super);

        Ok(self.parenthesize2(
            "super",
            &[
                AstFragment::Token(&inner.method()),
            ],
        )?)
    }

    // My note: untested.
    fn visit_this_expr(&mut self, _: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        Ok("&self".to_string()) // TO_DO: what was the intention?
    }

    fn visit_unary_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let unary = unwrap_expr!(expr, Unary);

        Ok(self.parenthesize(unary.operator().lexeme(), 
        &[Rc::clone(unary.right())])?)
    }

    // My note: untested.
    fn visit_variable_expr(&mut self, expr: Rc<expr::Expr>) -> Result<String, LoxRuntimeError> {
        let variable = unwrap_expr!(expr, Variable);

        Ok(variable.name().lexeme().to_string())
    }
}

impl stmt::Visitor<String> for AstPrinter {
    // My note: untested.
    fn visit_block_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let block = unwrap_stmt!(stmt, Block);

        let mut fragments = vec![];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            block.statements()
                .iter()
                .map(|arg| AstFragment::Stmt(arg.clone()))
        );

        Ok(self.parenthesize2("block", &fragments)?)
    }

    // My note: untested.
    fn visit_class_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let class = unwrap_stmt!(stmt, Class);

        let mut builder = String::new();
        builder.push_str("(class ");
        builder.push_str(class.name().lexeme());

        if let Some(superclass) = class.superclass() {
            builder.push_str(" < ");
            builder.push_str(&self.print_expression(Rc::clone(superclass))?);
        }

        for method in class.methods() {
            builder.push(' ');
            let stmt_function = Rc::new(stmt::Stmt::Function(method.as_ref().clone()));
            builder.push_str(&self.print_statement(stmt_function)?);
        }

        builder.push(')');
        Ok(builder)
    }

    // My note: untested.
    fn visit_expression_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let expression = unwrap_stmt!(stmt, Expression);

        Ok(self.parenthesize2(
            ";",
            &[
                AstFragment::Expr(Rc::clone(expression.expression())),
            ],
        )?)
    }

    // My note: untested.
    fn visit_function_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let function = unwrap_stmt!(stmt, Function);

        let mut builder = String::new();
        builder.push_str("(fun ");
        builder.push_str(function.name().lexeme());
        builder.push_str("(");

        for (i, param) in function.params().iter().enumerate() {
            if i != 0 { builder.push_str(" "); }
            builder.push_str(param.lexeme());
        }

        builder.push_str(") ");

        for body in function.body() {
            builder.push_str(&stmt::Stmt::accept_ref(Rc::clone(body), self)?);
        }

        builder.push_str(")");
        Ok(builder)
    }

    // My note: untested.
    fn visit_if_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, If);
        let binding = inner.condition();

        let mut fragments = vec![
            AstFragment::Expr(Rc::clone(binding)),
        ];

        fragments.push(AstFragment::Stmt(Rc::clone(inner.then_branch())));

        if let Some(else_branch) = inner.else_branch() {
            fragments.push(AstFragment::Stmt(Rc::clone(else_branch)));
        }

        let tag = if inner.else_branch().is_some() {
            "if-else"
        } else {
            "if"
        };

        Ok(self.parenthesize2(tag, &fragments)?)
    }

    // My note: untested.
    fn visit_print_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let print = unwrap_stmt!(stmt, Print);

        Ok(self.parenthesize2(
            "print",
            &[
                AstFragment::Expr(Rc::clone(print.expression())),
            ],
        )?)
    }

    // My note: untested.
    fn visit_return_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, Return);

        if let Some(value) = inner.value() {
            Ok(self.parenthesize2("return", &[AstFragment::Expr(value.clone())])?)
        } else {
            Ok("(return)".to_string())
        }
    }

    // My note: untested.
    fn visit_var_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let var = unwrap_stmt!(stmt, Var);

        let mut fragments = vec![
            AstFragment::Token(var.name()),
        ];

        if let Some(initializer) = var.initializer() {
            fragments.push(AstFragment::Text("=".to_string()));
            fragments.push(AstFragment::Expr(Rc::clone(initializer)));
        }

        Ok(self.parenthesize2("var", &fragments)?)
    }

    // My note: untested.
    fn visit_while_stmt(&mut self, stmt: Rc<stmt::Stmt>) -> Result<String, LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, While);

        Ok(self.parenthesize2(
            "while",
            &[
                AstFragment::Expr(Rc::clone(inner.condition())),
                AstFragment::Stmt(inner.body().clone()),
            ],
        )?)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::token_type::TokenType;
    use crate::token::{LiteralValue, Token};
    use super::expr;
    use super::expr::*;
    use crate::ast_printer::AstPrinter;

    #[test]
    // Please see the expression: (* (- 123) (group 45.67)), under:
    //     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
    fn it_works() {
        let expr = Rc::new(expr::Expr::Binary(Binary::new(
            Rc::new(expr::Expr::Unary(Unary::new(
                Token::new(TokenType::Minus, "-".to_string(), None, 1),
                Rc::new(expr::Expr::Literal(Literal::new(LiteralValue::Number(123.0)))),
            ))),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Rc::new(expr::Expr::Grouping(Grouping::new(
                Rc::new(expr::Expr::Literal(Literal::new(LiteralValue::Number(45.67)))),
            ))),
        )));

        assert_eq!("(* (- 123.0) (group 45.67))".to_string(), AstPrinter{}.print_expression(expr).unwrap());
    }
}

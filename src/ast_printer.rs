/* Date Created: 02/07/2025. */

//! The **A (Not Very) Pretty Printer** in  
//! [https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer](https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer)
//! 
//! The full original Java version can be found at:
//! https://github.com/munificent/craftinginterpreters/blob/master/java/com/craftinginterpreters/lox/AstPrinter.java

// To run test for this module only: 
// 
//     * cargo test ast_printer::tests

use super::lox_error::LoxError;
use super::lox_runtime_error::LoxRuntimeError;
use super::token::{LiteralValue, Token};

use super::expr;
use super::stmt;

// Rust-specific.
pub enum AstFragment<'a> {
    Expr(&'a expr::Expr),
    Stmt(&'a stmt::Stmt),
    Token(&'a Token),
    Text(String),
    Group(Vec<AstFragment<'a>>),
}

impl<'a> AstFragment<'a> {
    fn write_to(&self, printer: &mut AstPrinter, builder: &mut String) {
        builder.push(' ');
        match self {
            AstFragment::Expr(e) => builder.push_str(
                &e.accept_ref(printer).unwrap_or_else(|err| format!("[Error printing expression: {}]", err))
            ),
            AstFragment::Stmt(s) => builder.push_str(
                &s.accept_ref(printer).unwrap_or_else(|err| format!("[Error printing statement: {}]", err))
            ),
            AstFragment::Token(t) => builder.push_str(t.get_lexeme()),
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
    pub fn print_expression(&mut self, expr: &expr::Expr) -> Result<String, LoxRuntimeError> {
        expr.accept_ref(self)
    }

    pub fn print_statement(&mut self, stmt: &stmt::Stmt) -> Result<String, LoxRuntimeError> {
        stmt.accept_ref(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&expr::Expr]) -> Result<String, LoxError> {
        let mut builder = String::from(format!("({}", name));

        for e in exprs {
            builder.push_str(&format!(" {}", e.accept_ref(self)?));
        }        
        builder.push_str(")");

        Ok(builder)
    }

    fn transform(&mut self, builder: &mut String, parts: &[AstFragment]) {
        for part in parts {
            builder.push(' ');
            match part {
                AstFragment::Expr(expr) => builder.push_str(
                    &expr.accept_ref(self).unwrap_or_else(|err| format!("[Error printing expression: {}]", err))
                ),
                AstFragment::Stmt(stmt) => builder.push_str(
                    &stmt.accept_ref(self).unwrap_or_else(|err| format!("[Error printing statement: {}]", err))
                ),
                AstFragment::Token(token) => builder.push_str(token.get_lexeme()),
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
    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            &expr.get_name().get_lexeme(),
            &[
                AstFragment::Expr(&expr.get_value()),
            ],
        )?)
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize(expr.get_operator().get_lexeme(), 
        &[expr.get_left(), expr.get_right()])?)
    }    

    // My note: untested.
    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<String, LoxRuntimeError> {
        let mut fragments = vec![AstFragment::Expr(expr.get_callee())];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            expr.get_arguments()
                .iter()
                .map(|arg| AstFragment::Expr(arg))
        );

        Ok(self.parenthesize2("call", &fragments)?)
    }

    // My note: untested.
    fn visit_get_expr(&mut self, expr: &expr::Get) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            ".",
            &[
                AstFragment::Expr(&expr.get_object()),
                AstFragment::Text(expr.get_name().get_lexeme().to_string()),
            ],
        )?)
    }
    
    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize("group", &[expr.get_expression()])?)
    }

    fn visit_literal_expr(&mut self, expr: &expr::Literal) -> Result<String, LoxRuntimeError> {
        match expr.get_value() {
            LiteralValue::Number(n) => Ok(format!("{:?}", n)),
            LiteralValue::String(s) => Ok(s.to_string()),
            LiteralValue::Boolean(b) => Ok(b.to_string()),
            LiteralValue::Nil => Ok("nil".to_string()),
        }        
    }

    // My note: untested.
    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize(
            expr.get_operator().get_lexeme(), 
            &[
                        expr.get_left(),
                        expr.get_right(),
                    ]
        )?)
    }

    // My note: untested.
    fn visit_set_expr(&mut self, expr: &expr::Set) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            "=",
            &[
                AstFragment::Expr(&expr.get_object()),
                AstFragment::Text(expr.get_name().get_lexeme().to_string()),
                AstFragment::Expr(&expr.get_value()),
            ],
        )?)
    }

    // My note: untested.
    fn visit_super_expr(&mut self, expr: &expr::Super) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            "super",
            &[
                AstFragment::Token(&expr.get_method()),
            ],
        )?)
    }

    // My note: untested.
    fn visit_this_expr(&mut self, _: &expr::This) -> Result<String, LoxRuntimeError> {
        Ok("&self".to_string()) // TO_DO: what was the intention?
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize(expr.get_operator().get_lexeme(), 
        &[expr.get_right()])?)
    }

    // My note: untested.
    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<String, LoxRuntimeError> {
        Ok(expr.get_name().get_lexeme().to_string())
    }
}

impl stmt::Visitor<String> for AstPrinter {
    // My note: untested.
    fn visit_block_stmt(&mut self, stmt: &stmt::Block) -> Result<String, LoxRuntimeError> {
        let mut fragments = vec![];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            stmt.get_statements()
                .iter()
                .map(|arg| AstFragment::Stmt(arg))
        );

        Ok(self.parenthesize2("block", &fragments)?)
    }

    // My note: untested.
    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Result<String, LoxRuntimeError> {
        let mut builder = String::new();
        builder.push_str("(class ");
        builder.push_str(stmt.get_name().get_lexeme());

        if let Some(superclass) = stmt.get_superclass() {
            builder.push_str(" < ");
            builder.push_str(&self.print_expression(superclass)?);
        }

        for method in stmt.get_methods() {
            builder.push(' ');
            builder.push_str(&self.print_statement(&stmt::Stmt::Function(method.clone()))?);
        }

        builder.push(')');
        Ok(builder)
    }

    // My note: untested.
    fn visit_expression_stmt(&mut self, stmt: &stmt::Expression) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            ";",
            &[
                AstFragment::Expr(&stmt.get_expression()),
            ],
        )?)
    }

    // My note: untested.
    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Result<String, LoxRuntimeError> {
        let mut builder = String::new();
        builder.push_str("(fun ");
        builder.push_str(stmt.get_name().get_lexeme());
        builder.push_str("(");

        for (i, param) in stmt.get_params().iter().enumerate() {
            if i != 0 { builder.push_str(" "); }
            builder.push_str(param.get_lexeme());
        }

        builder.push_str(") ");

        for body in stmt.get_body() {
            builder.push_str(&body.accept_ref(self)?);
        }

        builder.push_str(")");
        Ok(builder)
    }

    // My note: untested.
    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Result<String, LoxRuntimeError> {
        let mut fragments = vec![
            AstFragment::Expr(stmt.get_condition()),
        ];

        fragments.push(AstFragment::Stmt(stmt.get_then_branch()));

        if let Some(else_branch) = stmt.get_else_branch() {
            fragments.push(AstFragment::Stmt(else_branch));
        }

        let tag = if stmt.get_else_branch().is_some() {
            "if-else"
        } else {
            "if"
        };

        Ok(self.parenthesize2(tag, &fragments)?)
    }

    // My note: untested.
    fn visit_print_stmt(&mut self, stmt: &stmt::Print) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            "print",
            &[
                AstFragment::Expr(&stmt.get_expression()),
            ],
        )?)
    }

    // My note: untested.
    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Result<String, LoxRuntimeError> {
        if let Some(value) = stmt.get_value() {
            Ok(self.parenthesize2("return", &[AstFragment::Expr(value)])?)
        } else {
            Ok("(return)".to_string())
        }
    }

    // My note: untested.
    fn visit_var_stmt(&mut self, stmt: &stmt::Var) -> Result<String, LoxRuntimeError> {
        let mut fragments = vec![
            AstFragment::Token(&stmt.get_name()),
        ];

        if let Some(initializer) = stmt.get_initializer() {
            fragments.push(AstFragment::Text("=".to_string()));
            fragments.push(AstFragment::Expr(initializer));
        }

        Ok(self.parenthesize2("var", &fragments)?)
    }

    // My note: untested.
    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Result<String, LoxRuntimeError> {
        Ok(self.parenthesize2(
            "while",
            &[
                AstFragment::Expr(&stmt.get_condition()),
                AstFragment::Stmt(&stmt.get_body()),
            ],
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::token_type::TokenType;
    use crate::token::{LiteralValue, Token};
    use super::expr;
    use super::expr::*;
    use crate::ast_printer::AstPrinter;

    #[test]
    // Please see the expression: (* (- 123) (group 45.67)), under:
    //     https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer
    fn it_works() {
        let expr = expr::Expr::Binary(Binary::new(
            expr::Expr::Unary(Unary::new(
                Token::new(TokenType::Minus, "-".to_string(), None, 1),
                expr::Expr::Literal(Literal::new(LiteralValue::Number(123.0))),
            )),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            expr::Expr::Grouping(Grouping::new(
                expr::Expr::Literal(Literal::new(LiteralValue::Number(45.67))),
            )),
        ));    

        assert_eq!("(* (- 123.0) (group 45.67))".to_string(), AstPrinter{}.print_expression(&expr).unwrap());
    }
}

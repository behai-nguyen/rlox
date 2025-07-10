/* Date Created: 02/07/2025. */

//! The **A (Not Very) Pretty Printer** in  
//! [https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer](https://craftinginterpreters.com/representing-code.html#a-not-very-pretty-printer)
//! 
//! The full original Java version can be found at:
//! https://github.com/munificent/craftinginterpreters/blob/master/java/com/craftinginterpreters/lox/AstPrinter.java

// To run test for this module only: 
// 
//     * cargo test ast_printer::tests

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
    fn write_to(&self, printer: &AstPrinter, builder: &mut String) {
        builder.push(' ');
        match self {
            AstFragment::Expr(e) => builder.push_str(&e.accept(printer)),
            AstFragment::Stmt(s) => builder.push_str(&s.accept(printer)),
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
    pub fn print_expression(&self, expr: &expr::Expr) -> String {
        expr.accept(self)
    }

    pub fn print_statement(&self, stmt: &stmt::Stmt) -> String {
        stmt.accept(self)
    }    

    fn parenthesize(&self, name: &str, exprs: &[&expr::Expr]) -> String {
        let mut builder = String::from(format!("({}", name));

        for e in exprs {
            builder.push_str(&format!(" {}", e.accept(self)));
        }        
        builder.push_str(")");

        builder
    }

    fn transform(&self, builder: &mut String, parts: &[AstFragment]) {
        for part in parts {
            builder.push(' ');
            match part {
                AstFragment::Expr(expr) => builder.push_str(&expr.accept(self)),
                AstFragment::Stmt(stmt) => builder.push_str(&stmt.accept(self)),
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
    fn parenthesize2(&self, name: &str, parts: &[AstFragment]) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        self.transform(&mut builder, parts);
        builder.push(')');
        builder
    }

}

impl expr::Visitor<String> for AstPrinter {
    // My note: untested.
    fn visit_assign_expr(&self, expr: &expr::Assign) -> String {
        self.parenthesize2(
            &expr.get_name().get_lexeme(),
            &[
                AstFragment::Expr(&expr.get_value()),
            ],
        )        
    }

    fn visit_binary_expr(&self, expr: &expr::Binary) -> String {
        self.parenthesize(expr.get_operator().get_lexeme(), 
        &[expr.get_left(), expr.get_right()])
    }    
    /*
    // My note: this version also works.
    fn visit_binary_expr(&self, expr: &expr::Binary) -> String {
        self.parenthesize2(
            &expr.get_operator().get_lexeme(),
            &[
                AstFragment::Expr(&expr.get_left()),
                AstFragment::Expr(&expr.get_right()),
            ],
        )
    }
    */

    // My note: untested.
    fn visit_call_expr(&self, expr: &expr::Call) -> String {
        let mut fragments = vec![AstFragment::Expr(expr.get_callee())];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            expr.get_arguments()
                .iter()
                .map(|arg| AstFragment::Expr(arg))
        );

        self.parenthesize2("call", &fragments)
    }

    // My note: untested.
    fn visit_get_expr(&self, expr: &expr::Get) -> String {
        self.parenthesize2(
            ".",
            &[
                AstFragment::Expr(&expr.get_object()),
                AstFragment::Text(expr.get_name().get_lexeme().to_string()),
            ],
        )
    }
    
    fn visit_grouping_expr(&self, expr: &expr::Grouping) -> String {
        self.parenthesize("group", &[expr.get_expression()])
    }
    /*
    // My note: this version also works.
    fn visit_grouping_expr(&self, expr: &expr::Grouping) -> String {
        self.parenthesize2(
            "group",
            &[
                AstFragment::Expr(&expr.get_expression()),
            ],
        )
    }
    */

    fn visit_literal_expr(&self, expr: &expr::Literal) -> String {
        match expr.get_value() {
            LiteralValue::Number(n) => format!("{:?}", n),
            LiteralValue::String(s) => s.to_string(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }        
    }

    // My note: untested.
    fn visit_logical_expr(&self, expr: &expr::Logical) -> String {
        self.parenthesize(
            expr.get_operator().get_lexeme(), 
            &[
                        expr.get_left(),
                        expr.get_right(),
                    ]
        )
    }

    // My note: untested.
    fn visit_set_expr(&self, expr: &expr::Set) -> String {
        self.parenthesize2(
            "=",
            &[
                AstFragment::Expr(&expr.get_object()),
                AstFragment::Text(expr.get_name().get_lexeme().to_string()),
                AstFragment::Expr(&expr.get_value()),
            ],
        )
    }

    // My note: untested.
    fn visit_super_expr(&self, expr: &expr::Super) -> String {
        self.parenthesize2(
            "super",
            &[
                AstFragment::Token(&expr.get_method()),
            ],
        )        
    }

    // My note: untested.
    fn visit_this_expr(&self, _: &expr::This) -> String {
        "&self".to_string()
    }

    fn visit_unary_expr(&self, expr: &expr::Unary) -> String {
        AstPrinter{}.parenthesize(expr.get_operator().get_lexeme(), 
        &[expr.get_right()])        
    }
    /*
    // My note: this version also works.
    fn visit_unary_expr(&self, expr: &expr::Unary) -> String {
        self.parenthesize2(
            expr.get_operator().get_lexeme(),
            &[
                AstFragment::Expr(&expr.get_right()),
            ],
        )
    }
    */

    // My note: untested.
    fn visit_variable_expr(&self, expr: &expr::Variable) -> String {
        expr.get_name().get_lexeme().to_string()
    }
}

impl stmt::Visitor<String> for AstPrinter {
    // My note: untested.
    fn visit_block_stmt(&self, stmt: &stmt::Block) -> String {
        let mut fragments = vec![];

        // Map each argument into an AstFragment::Expr and extend the list
        fragments.extend(
            stmt.get_statements()
                .iter()
                .map(|arg| AstFragment::Stmt(arg))
        );

        self.parenthesize2("block", &fragments)
    }

    // My note: untested.
    fn visit_class_stmt(&self, stmt: &stmt::Class) -> String {
        let mut builder = String::new();
        builder.push_str("(class ");
        builder.push_str(stmt.get_name().get_lexeme());

        if let Some(superclass) = stmt.get_superclass() {
            builder.push_str(" < ");
            builder.push_str(&self.print_expression(superclass));
        }

        for method in stmt.get_methods() {
            builder.push(' ');
            builder.push_str(&self.print_statement(&stmt::Stmt::Function(method.clone())));
        }

        builder.push(')');
        builder
    }

    // My note: untested.
    fn visit_expression_stmt(&self, stmt: &stmt::Expression) -> String {
        self.parenthesize2(
            ";",
            &[
                AstFragment::Expr(&stmt.get_expression()),
            ],
        )
    }

    // My note: untested.
    fn visit_function_stmt(&self, stmt: &stmt::Function) -> String {
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
            builder.push_str(&body.accept(self));
        }

        builder.push_str(")");
        builder
    }

    // My note: untested.
    fn visit_if_stmt(&self, stmt: &stmt::If) -> String {
        let mut fragments = vec![
            AstFragment::Expr(stmt.get_condition()),
        ];

        if let Some(then_branch) = stmt.get_then_branch() {
            fragments.push(AstFragment::Stmt(then_branch));
        }

        if let Some(else_branch) = stmt.get_else_branch() {
            fragments.push(AstFragment::Stmt(else_branch));
        }

        let tag = if stmt.get_else_branch().is_some() {
            "if-else"
        } else {
            "if"
        };

        self.parenthesize2(tag, &fragments)
    }

    // My note: untested.
    fn visit_print_stmt(&self, stmt: &stmt::Print) -> String {
        self.parenthesize2(
            "print",
            &[
                AstFragment::Expr(&stmt.get_expression()),
            ],
        )
    }

    // My note: untested.
    fn visit_return_stmt(&self, stmt: &stmt::Return) -> String {
        if let Some(value) = stmt.get_value() {
            self.parenthesize2("return", &[AstFragment::Expr(value)])
        } else {
            "(return)".to_string()
        }
    }

    // My note: untested.
    fn visit_var_stmt(&self, stmt: &stmt::Var) -> String {
        let mut fragments = vec![
            AstFragment::Token(&stmt.get_name()),
        ];

        if let Some(initializer) = stmt.get_initializer() {
            fragments.push(AstFragment::Text("=".to_string()));
            fragments.push(AstFragment::Expr(initializer));
        }

        self.parenthesize2("var", &fragments)
    }

    // My note: untested.
    fn visit_while_stmt(&self, stmt: &stmt::While) -> String {
        self.parenthesize2(
            "while",
            &[
                AstFragment::Expr(&stmt.get_condition()),
                AstFragment::Stmt(&stmt.get_body()),
            ],
        )
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

        assert_eq!("(* (- 123.0) (group 45.67))".to_string(), AstPrinter{}.print_expression(&expr));
    }
}

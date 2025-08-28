/// Appendix II expr
use std::rc::Rc;

use super::token::{LiteralValue, Token};
use super::lox_runtime_error::LoxRuntimeError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Assign {
    name: Token,
    value: Rc<Expr>,
}

impl Assign {
    pub fn new(name: Token, 
        value: Rc<Expr>
    ) -> Self {
        Assign {
            name,
            value,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Rc<Expr> {
        &self.value
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Binary {
    left: Rc<Expr>,
    operator: Token,
    right: Rc<Expr>,
}

impl Binary {
    pub fn new(left: Rc<Expr>, 
        operator: Token, 
        right: Rc<Expr>
    ) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Rc<Expr> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
        &self.right
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Call {
    callee: Rc<Expr>,
    paren: Token,
    arguments: Vec<Rc<Expr>>,
}

impl Call {
    pub fn new(callee: Rc<Expr>, 
        paren: Token, 
        arguments: Vec<Rc<Expr>>
    ) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }

    pub fn callee(&self) -> &Rc<Expr> {
        &self.callee
    }

    pub fn paren(&self) -> &Token {
        &self.paren
    }

    pub fn arguments(&self) -> &Vec<Rc<Expr>> {
        &self.arguments
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Get {
    object: Rc<Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Rc<Expr>, 
        name: Token
    ) -> Self {
        Get {
            object,
            name,
        }
    }

    pub fn object(&self) -> &Rc<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Grouping {
    expression: Rc<Expr>,
}

impl Grouping {
    pub fn new(expression: Rc<Expr>) -> Self {
        Grouping {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
        &self.expression
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Literal {
    value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal {
            value,
        }
    }

    pub fn value(&self) -> &LiteralValue {
        &self.value
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Logical {
    left: Rc<Expr>,
    operator: Token,
    right: Rc<Expr>,
}

impl Logical {
    pub fn new(left: Rc<Expr>, 
        operator: Token, 
        right: Rc<Expr>
    ) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &Rc<Expr> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
        &self.right
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Set {
    object: Rc<Expr>,
    name: Token,
    value: Rc<Expr>,
}

impl Set {
    pub fn new(object: Rc<Expr>, 
        name: Token, 
        value: Rc<Expr>
    ) -> Self {
        Set {
            object,
            name,
            value,
        }
    }

    pub fn object(&self) -> &Rc<Expr> {
        &self.object
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &Rc<Expr> {
        &self.value
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub fn new(keyword: Token, 
        method: Token
    ) -> Self {
        Super {
            keyword,
            method,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn method(&self) -> &Token {
        &self.method
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct This {
    keyword: Token,
}

impl This {
    pub fn new(keyword: Token) -> Self {
        This {
            keyword,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Unary {
    operator: Token,
    right: Rc<Expr>,
}

impl Unary {
    pub fn new(operator: Token, 
        right: Rc<Expr>
    ) -> Self {
        Unary {
            operator,
            right,
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Rc<Expr> {
        &self.right
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Variable {
            name,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

}

// Define enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

// Visitor Trait
pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_binary_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_call_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_get_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_grouping_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_literal_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_logical_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_set_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_super_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_this_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_unary_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
    fn visit_variable_expr(&mut self, expr: Rc<Expr>) -> Result<T, LoxRuntimeError>;
}

// Implement `accept()` for `Expr`.
impl Expr {
    pub fn accept<T>(expr: Rc<Expr>, visitor: &mut dyn Visitor<T>) -> Result<T, LoxRuntimeError> {
        match expr.as_ref() {
            Expr::Assign(_) => visitor.visit_assign_expr(expr),
            Expr::Binary(_) => visitor.visit_binary_expr(expr),
            Expr::Call(_) => visitor.visit_call_expr(expr),
            Expr::Get(_) => visitor.visit_get_expr(expr),
            Expr::Grouping(_) => visitor.visit_grouping_expr(expr),
            Expr::Literal(_) => visitor.visit_literal_expr(expr),
            Expr::Logical(_) => visitor.visit_logical_expr(expr),
            Expr::Set(_) => visitor.visit_set_expr(expr),
            Expr::Super(_) => visitor.visit_super_expr(expr),
            Expr::This(_) => visitor.visit_this_expr(expr),
            Expr::Unary(_) => visitor.visit_unary_expr(expr),
            Expr::Variable(_) => visitor.visit_variable_expr(expr),
        }
    }
}
//< Appendix II expr

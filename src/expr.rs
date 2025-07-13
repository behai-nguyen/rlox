//> Appendix II expr
use super::token::{LiteralValue, Token};
use super::lox_error::LoxError;

//> expr-assign
#[derive(Clone)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}

impl Assign {
    pub fn new(name: Token, 
        value: Box<Expr>
    ) -> Self {
        Assign {
            name,
            value,
        }
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

    pub fn get_value(&self) -> &Box<Expr> {
        &self.value
    }

}

//> expr-binary
#[derive(Clone)]
pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, 
        operator: Token, 
        right: Expr
    ) -> Self {
        Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn get_left(&self) -> &Box<Expr> {
        &self.left
    }

    pub fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub fn get_right(&self) -> &Box<Expr> {
        &self.right
    }

}

//> expr-call
#[derive(Clone)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}

impl Call {
    pub fn new(callee: Expr, 
        paren: Token, 
        arguments: Vec<Expr>
    ) -> Self {
        Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    pub fn get_callee(&self) -> &Box<Expr> {
        &self.callee
    }

    pub fn get_paren(&self) -> &Token {
        &self.paren
    }

    pub fn get_arguments(&self) -> &Vec<Expr> {
        &self.arguments
    }

}

//> expr-get
#[derive(Clone)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Expr, 
        name: Token
    ) -> Self {
        Get {
            object: Box::new(object),
            name,
        }
    }

    pub fn get_object(&self) -> &Box<Expr> {
        &self.object
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

}

//> expr-grouping
#[derive(Clone)]
pub struct Grouping {
    expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Self {
        Grouping {
            expression: Box::new(expression),
        }
    }

    pub fn get_expression(&self) -> &Box<Expr> {
        &self.expression
    }

}

//> expr-literal
#[derive(Clone)]
pub struct Literal {
    value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal {
            value,
        }
    }

    pub fn get_value(&self) -> &LiteralValue {
        &self.value
    }

}

//> expr-logical
#[derive(Clone)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl Logical {
    pub fn new(left: Expr, 
        operator: Token, 
        right: Expr
    ) -> Self {
        Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn get_left(&self) -> &Box<Expr> {
        &self.left
    }

    pub fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub fn get_right(&self) -> &Box<Expr> {
        &self.right
    }

}

//> expr-set
#[derive(Clone)]
pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}

impl Set {
    pub fn new(object: Expr, 
        name: Token, 
        value: Expr
    ) -> Self {
        Set {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }

    pub fn get_object(&self) -> &Box<Expr> {
        &self.object
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

    pub fn get_value(&self) -> &Box<Expr> {
        &self.value
    }

}

//> expr-super
#[derive(Clone)]
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

    pub fn get_keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn get_method(&self) -> &Token {
        &self.method
    }

}

//> expr-this
#[derive(Clone)]
pub struct This {
    keyword: Token,
}

impl This {
    pub fn new(keyword: Token) -> Self {
        This {
            keyword,
        }
    }

    pub fn get_keyword(&self) -> &Token {
        &self.keyword
    }

}

//> expr-unary
#[derive(Clone)]
pub struct Unary {
    operator: Token,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, 
        right: Expr
    ) -> Self {
        Unary {
            operator,
            right: Box::new(right),
        }
    }

    pub fn get_operator(&self) -> &Token {
        &self.operator
    }

    pub fn get_right(&self) -> &Box<Expr> {
        &self.right
    }

}

//> expr-variable
#[derive(Clone)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Variable {
            name,
        }
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

}

// Define enum
#[derive(Clone)]
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
    fn visit_assign_expr(&self, expr: &Assign) -> Result<T, LoxError>;
    fn visit_binary_expr(&self, expr: &Binary) -> Result<T, LoxError>;
    fn visit_call_expr(&self, expr: &Call) -> Result<T, LoxError>;
    fn visit_get_expr(&self, expr: &Get) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, expr: &Literal) -> Result<T, LoxError>;
    fn visit_logical_expr(&self, expr: &Logical) -> Result<T, LoxError>;
    fn visit_set_expr(&self, expr: &Set) -> Result<T, LoxError>;
    fn visit_super_expr(&self, expr: &Super) -> Result<T, LoxError>;
    fn visit_this_expr(&self, expr: &This) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, expr: &Unary) -> Result<T, LoxError>;
    fn visit_variable_expr(&self, expr: &Variable) -> Result<T, LoxError>;
}

// Implement `accept()` for `Expr`
impl Expr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Assign(val) => visitor.visit_assign_expr(val),
            Expr::Binary(val) => visitor.visit_binary_expr(val),
            Expr::Call(val) => visitor.visit_call_expr(val),
            Expr::Get(val) => visitor.visit_get_expr(val),
            Expr::Grouping(val) => visitor.visit_grouping_expr(val),
            Expr::Literal(val) => visitor.visit_literal_expr(val),
            Expr::Logical(val) => visitor.visit_logical_expr(val),
            Expr::Set(val) => visitor.visit_set_expr(val),
            Expr::Super(val) => visitor.visit_super_expr(val),
            Expr::This(val) => visitor.visit_this_expr(val),
            Expr::Unary(val) => visitor.visit_unary_expr(val),
            Expr::Variable(val) => visitor.visit_variable_expr(val),
        }
    }
}
//< Appendix II expr

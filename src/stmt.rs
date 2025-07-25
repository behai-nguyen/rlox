//> Appendix II stmt
use super::token::Token;
use super::expr::Expr;
use super::lox_error::LoxError;

//> stmt-block
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block {
            statements,
        }
    }

    pub fn get_statements(&self) -> &Vec<Stmt> {
        &self.statements
    }

}

//> stmt-class
#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    name: Token,
    superclass: Option<Expr>,
    methods: Vec<Function>,
}

impl Class {
    pub fn new(name: Token, 
        superclass: Option<Expr>, 
        methods: Vec<Function>
    ) -> Self {
        Class {
            name,
            superclass,
            methods,
        }
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

    pub fn get_superclass(&self) -> &Option<Expr> {
        &self.superclass
    }

    pub fn get_methods(&self) -> &Vec<Function> {
        &self.methods
    }

}

//> stmt-expression
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Self {
        Expression {
            expression,
        }
    }

    pub fn get_expression(&self) -> &Expr {
        &self.expression
    }

}

//> stmt-function
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, 
        params: Vec<Token>, 
        body: Vec<Stmt>
    ) -> Self {
        Function {
            name,
            params,
            body,
        }
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

    pub fn get_params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn get_body(&self) -> &Vec<Stmt> {
        &self.body
    }

}

//> stmt-if
#[derive(Debug, Clone, PartialEq)]
pub struct If {
    condition: Expr,
    then_branch: Box<Stmt>,
    else_branch: Option<Box<Stmt>>,
}

impl If {
    pub fn new(condition: Expr, 
        then_branch: Stmt, 
        else_branch: Option<Stmt>
    ) -> Self {
        If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub fn get_condition(&self) -> &Expr {
        &self.condition
    }

    pub fn get_then_branch(&self) -> &Box<Stmt> {
        &self.then_branch
    }

    pub fn get_else_branch(&self) -> &Option<Box<Stmt>> {
        &self.else_branch
    }

}

//> stmt-print
#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Self {
        Print {
            expression,
        }
    }

    pub fn get_expression(&self) -> &Expr {
        &self.expression
    }

}

//> stmt-return
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    keyword: Token,
    value: Option<Expr>,
}

impl Return {
    pub fn new(keyword: Token, 
        value: Option<Expr>
    ) -> Self {
        Return {
            keyword,
            value,
        }
    }

    pub fn get_keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn get_value(&self) -> &Option<Expr> {
        &self.value
    }

}

//> stmt-var
#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    name: Token,
    initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: Token, 
        initializer: Option<Expr>
    ) -> Self {
        Var {
            name,
            initializer,
        }
    }

    pub fn get_name(&self) -> &Token {
        &self.name
    }

    pub fn get_initializer(&self) -> &Option<Expr> {
        &self.initializer
    }

}

//> stmt-while
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    condition: Expr,
    body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, 
        body: Stmt
    ) -> Self {
        While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn get_condition(&self) -> &Expr {
        &self.condition
    }

    pub fn get_body(&self) -> &Box<Stmt> {
        &self.body
    }

}

// Define enum
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}

// Visitor Trait
pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<T, LoxError>;
    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<T, LoxError>;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<T, LoxError>;
    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<T, LoxError>;
    fn visit_if_stmt(&mut self, stmt: &If) -> Result<T, LoxError>;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<T, LoxError>;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<T, LoxError>;
    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<T, LoxError>;
    fn visit_while_stmt(&mut self, stmt: &While) -> Result<T, LoxError>;
}

// Implement `accept()`, `accept_ref()` for `Stmt`
impl Stmt {
    pub fn accept<T>(&mut self, visitor: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(val) => visitor.visit_block_stmt(val),
            Stmt::Class(val) => visitor.visit_class_stmt(val),
            Stmt::Expression(val) => visitor.visit_expression_stmt(val),
            Stmt::Function(val) => visitor.visit_function_stmt(val),
            Stmt::If(val) => visitor.visit_if_stmt(val),
            Stmt::Print(val) => visitor.visit_print_stmt(val),
            Stmt::Return(val) => visitor.visit_return_stmt(val),
            Stmt::Var(val) => visitor.visit_var_stmt(val),
            Stmt::While(val) => visitor.visit_while_stmt(val),
        }
    }

    pub fn accept_ref<T>(&self, visitor: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Block(val) => visitor.visit_block_stmt(val),
            Stmt::Class(val) => visitor.visit_class_stmt(val),
            Stmt::Expression(val) => visitor.visit_expression_stmt(val),
            Stmt::Function(val) => visitor.visit_function_stmt(val),
            Stmt::If(val) => visitor.visit_if_stmt(val),
            Stmt::Print(val) => visitor.visit_print_stmt(val),
            Stmt::Return(val) => visitor.visit_return_stmt(val),
            Stmt::Var(val) => visitor.visit_var_stmt(val),
            Stmt::While(val) => visitor.visit_while_stmt(val),
        }
    }
}
//< Appendix II stmt

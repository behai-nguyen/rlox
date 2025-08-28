/// Appendix II stmt
use std::rc::Rc;

use super::token::Token;
use super::expr::Expr;
use super::lox_runtime_error::LoxRuntimeError;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Block {
    statements: Vec<Rc<Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Rc<Stmt>>) -> Self {
        Block {
            statements,
        }
    }

    pub fn statements(&self) -> &Vec<Rc<Stmt>> {
        &self.statements
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Class {
    name: Token,
    superclass: Option<Rc<Expr>>,
    methods: Vec<Rc<Function>>,
}

impl Class {
    pub fn new(name: Token, 
        superclass: Option<Rc<Expr>>, 
        methods: Vec<Rc<Function>>
    ) -> Self {
        Class {
            name,
            superclass,
            methods,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn superclass(&self) -> &Option<Rc<Expr>> {
        &self.superclass
    }

    pub fn methods(&self) -> &Vec<Rc<Function>> {
        &self.methods
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Expression {
    expression: Rc<Expr>,
}

impl Expression {
    pub fn new(expression: Rc<Expr>) -> Self {
        Expression {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
        &self.expression
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<Rc<Stmt>>,
}

impl Function {
    pub fn new(name: Token, 
        params: Vec<Token>, 
        body: Vec<Rc<Stmt>>
    ) -> Self {
        Function {
            name,
            params,
            body,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn body(&self) -> &Vec<Rc<Stmt>> {
        &self.body
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct If {
    condition: Rc<Expr>,
    then_branch: Rc<Stmt>,
    else_branch: Option<Rc<Stmt>>,
}

impl If {
    pub fn new(condition: Rc<Expr>, 
        then_branch: Rc<Stmt>, 
        else_branch: Option<Rc<Stmt>>
    ) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn condition(&self) -> &Rc<Expr> {
        &self.condition
    }

    pub fn then_branch(&self) -> &Rc<Stmt> {
        &self.then_branch
    }

    pub fn else_branch(&self) -> &Option<Rc<Stmt>> {
        &self.else_branch
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Print {
    expression: Rc<Expr>,
}

impl Print {
    pub fn new(expression: Rc<Expr>) -> Self {
        Print {
            expression,
        }
    }

    pub fn expression(&self) -> &Rc<Expr> {
        &self.expression
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Return {
    keyword: Token,
    value: Option<Rc<Expr>>,
}

impl Return {
    pub fn new(keyword: Token, 
        value: Option<Rc<Expr>>
    ) -> Self {
        Return {
            keyword,
            value,
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn value(&self) -> &Option<Rc<Expr>> {
        &self.value
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Var {
    name: Token,
    initializer: Option<Rc<Expr>>,
}

impl Var {
    pub fn new(name: Token, 
        initializer: Option<Rc<Expr>>
    ) -> Self {
        Var {
            name,
            initializer,
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Option<Rc<Expr>> {
        &self.initializer
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct While {
    condition: Rc<Expr>,
    body: Rc<Stmt>,
}

impl While {
    pub fn new(condition: Rc<Expr>, 
        body: Rc<Stmt>
    ) -> Self {
        While {
            condition,
            body,
        }
    }

    pub fn condition(&self) -> &Rc<Expr> {
        &self.condition
    }

    pub fn body(&self) -> &Rc<Stmt> {
        &self.body
    }

}

// Define enum
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
    fn visit_block_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_class_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_expression_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_function_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_if_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_print_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_return_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_var_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
    fn visit_while_stmt(&mut self, stmt: Rc<Stmt>) -> Result<T, LoxRuntimeError>;
}

// Implement `accept()` for `Stmt`.
impl Stmt {
    pub fn accept<T>(stmt: Rc<Stmt>, visitor: &mut dyn Visitor<T>) -> Result<T, LoxRuntimeError> {
        match stmt.as_ref() {
            Stmt::Block(_) => visitor.visit_block_stmt(stmt),
            Stmt::Class(_) => visitor.visit_class_stmt(stmt),
            Stmt::Expression(_) => visitor.visit_expression_stmt(stmt),
            Stmt::Function(_) => visitor.visit_function_stmt(stmt),
            Stmt::If(_) => visitor.visit_if_stmt(stmt),
            Stmt::Print(_) => visitor.visit_print_stmt(stmt),
            Stmt::Return(_) => visitor.visit_return_stmt(stmt),
            Stmt::Var(_) => visitor.visit_var_stmt(stmt),
            Stmt::While(_) => visitor.visit_while_stmt(stmt),
        }
    }
}
//< Appendix II stmt

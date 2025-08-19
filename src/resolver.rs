/* Date Created: 02/08/2025. */

//! The **A Resolver Class** section in  
//! [https://craftinginterpreters.com/resolving-and-binding.html](https://craftinginterpreters.com/resolving-and-binding.html#a-resolver-class). 
//! 

use std::rc::Rc;
use std::collections::HashMap;

use super::interpreter::Interpreter;
use super::lox_runtime_error::LoxRuntimeError;
use super::lox_error_helper::error;
use super::{expr, expr::Expr};
use super::{stmt, stmt::{Stmt, Function}};
use super::{unwrap_expr, unwrap_stmt};
use super::token::Token;

#[derive(Copy, Clone, PartialEq)]
enum FunctionType {
    Nil,
    Function,
    Initializer,
    Method,
}

#[derive(Copy, Clone, PartialEq)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter, 
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::Nil,
            current_class: ClassType::None,
        }
    }

    fn resolve_statement(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        Stmt::accept_ref(stmt, self)
    }

    fn resolve_expression(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        Expr::accept_ref(expr, self)
    }

    pub fn resolve(&mut self, statements: &Vec<Rc<Stmt>>) -> Result<(), LoxRuntimeError> {
        for stmt in statements {
            self.resolve_statement(Rc::clone(stmt))?
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("Tried to end scope when none was active");
    }

    fn declare(&mut self, name: &Token) -> Result<(), LoxRuntimeError> {
        if let Some(scope) = self.scopes.last_mut() {
            let lexeme = name.lexeme();

            if scope.contains_key(lexeme) {
                return Err(LoxRuntimeError::Error(error(name, "Already a variable with this name in this scope.")));
            }
            scope.insert(lexeme.to_string(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme().to_string(), true);
        }
    }

    fn resolve_local(&mut self, expr: Rc<Expr>, name: &Token) {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(name.lexeme()) {
                let depth = self.scopes.len() - 1 - i;
                self.interpreter.resolve(expr, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &Function,
        func_type: FunctionType) -> Result<(), LoxRuntimeError> {
    
        let enclosing_function: FunctionType = self.current_function;
        self.current_function = func_type;

        self.begin_scope();
        for param in function.params() {
            self.declare(param)?;
            self.define(param);
        }

        let res = self.resolve(function.body());
        self.end_scope();
        
        self.current_function = enclosing_function;
        res
    }
}

impl<'a> expr::Visitor<()> for Resolver<'a> {
    fn visit_assign_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let binding = expr.clone();
        let assign = unwrap_expr!(binding, Assign);
        self.resolve_expression(Rc::clone(assign.value()))?;
        self.resolve_local(expr, assign.name());

        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let binary = unwrap_expr!(expr, Binary);
        self.resolve_expression(Rc::clone(binary.left()))?;
        self.resolve_expression(Rc::clone(binary.right()))?;

        Ok(())
    }

    fn visit_call_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let call = unwrap_expr!(expr, Call);
        self.resolve_expression(Rc::clone(call.callee()))?;

        for argument in call.arguments() {
            self.resolve_expression(Rc::clone(argument))?;
        };

        Ok(())
    }

    fn visit_get_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let get = unwrap_expr!(expr, Get);
        self.resolve_expression(Rc::clone(get.object()))?;
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let grouping = unwrap_expr!(expr, Grouping);
        self.resolve_expression(Rc::clone(grouping.expression()))?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, _: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let logical = unwrap_expr!(expr, Logical);

        self.resolve_expression(Rc::clone(logical.left()))?;
        self.resolve_expression(Rc::clone(logical.right()))?;

        Ok(())
    }

    fn visit_set_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let set = unwrap_expr!(expr, Set);

        self.resolve_expression(Rc::clone(set.value()))?;
        self.resolve_expression(Rc::clone(set.object()))?;

        Ok(())
    }

    fn visit_super_expr(&mut self, _: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        unimplemented!()
    }

    fn visit_this_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let binding = expr.clone();
        let this = unwrap_expr!(binding, This);

        if self.current_class == ClassType::None {
            return Err(LoxRuntimeError::Error(
                error(this.keyword(), "Can't use 'this' outside of a class."))
            );
        }

        self.resolve_local(expr, this.keyword());

        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let unary = unwrap_expr!(expr, Unary);

        self.resolve_expression(Rc::clone(unary.right()))?;
        Ok(())
    }

    fn visit_variable_expr(&mut self, expr: Rc<Expr>) -> Result<(), LoxRuntimeError> {
        let binding = expr.clone();
        let variable = unwrap_expr!(binding, Variable);

        if let Some(scope) = self.scopes.last() {
            if let Some(&false) = scope.get(variable.name().lexeme()) {
                return Err(LoxRuntimeError::Error(
                    error(variable.name(), "Can't read local variable in its own initializer."))
                );
            }
        }

        self.resolve_local(expr, variable.name());
        Ok(())
    }
}

impl<'a> stmt::Visitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let block = unwrap_stmt!(stmt, Block);

        self.begin_scope();
        let res = self.resolve(block.statements());
        self.end_scope();
        res
    }

    fn visit_class_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let class = unwrap_stmt!(stmt, Class);

        let enclosing_class: ClassType = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(class.name())?;
        self.define(class.name());

        self.begin_scope();
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert("this".to_string(), true);
        }

        for method in class.methods() {
            let mut declaration: FunctionType = FunctionType::Method;

            // We use the visited method’s name to determine if we’re resolving 
            // an initializer or not.
            if method.name().lexeme() == "init" {                
                declaration =  FunctionType::Initializer;
            }

            self.resolve_function(method, declaration)?;
        }

        self.end_scope();

        self.current_class = enclosing_class;

        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let expression = unwrap_stmt!(stmt, Expression);
        self.resolve_expression(Rc::clone(expression.expression()))
    }

    fn visit_function_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let function = unwrap_stmt!(stmt, Function);

        self.declare(function.name())?;
        self.define(function.name());

        self.resolve_function(function, FunctionType::Function)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, If);

        self.resolve_expression(Rc::clone(inner.condition()))?;
        self.resolve_statement(Rc::clone(inner.then_branch()))?;
        if let Some(else_stmt) = inner.else_branch() {
            self.resolve_statement(Rc::clone(else_stmt))?;            
        };

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let print = unwrap_stmt!(stmt, Print);
        let expr = print.expression();
        self.resolve_expression(Rc::clone(expr))?;        

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, Return);

        if self.current_function == FunctionType::Nil {
            return Err(LoxRuntimeError::Error(
                error(inner.keyword(), "Can't return from top-level code."))
            );
        }

        if let Some(expr) = inner.value() {
            if self.current_function == FunctionType::Initializer {
                return Err(LoxRuntimeError::Error(
                    error(inner.keyword(), "Can't return a value from an initializer."))
                );
            }
            self.resolve_expression(Rc::clone(expr))?;
        };

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let var = unwrap_stmt!(stmt, Var);

        self.declare(var.name())?;
        if let Some(expr) = var.initializer() {
            self.resolve_expression(Rc::clone(expr))?;
        };

        self.define(var.name());
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, While);

        self.resolve_expression(Rc::clone(inner.condition()))?;
        self.resolve_statement(Rc::clone(inner.body()))
    }
}

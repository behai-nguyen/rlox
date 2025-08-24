/* Date Created: 10/07/2025. */

//! The **Evaluating Expressions** section in  
//! [https://craftinginterpreters.com/evaluating-expressions.html](https://craftinginterpreters.com/evaluating-expressions.html). 
//! 

use std::io::Write;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;
use std::collections::HashMap;

use crate::token_type::TokenType;
use super::lox_error::LoxError;
use super::lox_runtime_error::LoxRuntimeError;
use super::lox_error_helper::{error, sys_error, runtime_error}; 
use super::token::{LiteralValue, Token};
use super::value::Value;
use super::{expr, expr::Expr};
use super::{stmt, stmt::Stmt};
use super::{unwrap_expr, unwrap_stmt};
use super::environment::{Environment, EnvironmentRef};

use super::lox_clock::LoxClock;
use super::lox_function::LoxFunction;
use super::lox_return::LoxReturn;

use super::lox_class::{LoxClass, LoxFunctionsMap};
use super::lox_instance::LoxInstance;

// Remove generic from Interpreter to enable src/lox_function.rs' 
// LoxFunction::call() to write the Interpreter::output.
//
// Whomever implements LoxCallable need to access Interpreter::output to 
// correctly writes the output to.
//
// ➜ LoxFunction implements LoxCallable.
//
// To enable working with Rc<dyn LoxCallable>: work around the trait object 
// constraints: object safety.
//
// For testing, we still need tests to write to byte streams. Writable is a type
// erasure trait: hiding the concrete type behind a trait object. Code can operate 
// on many different types without knowing what those types are at compile time. 
// 
// Writable::as_any() method provides a way to recover the original type at runtime 
// using downcasting: 
//
//     if let Some(cursor) = w.as_any().downcast_ref::<std::io::Cursor<Vec<u8>>>() {
//         // use cursor
//     }
// 
pub trait Writable: std::io::Write {
    #[allow(dead_code)]
    // Used in tests. Rust does not see it since test modules are not in
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: std::io::Write + Any> Writable for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Interpreter {
    output: Box<dyn Writable>,
    // The outermost scope.
    globals: EnvironmentRef,
    // The current scope.
    environment: EnvironmentRef,
    // Pointer identity: raw pointer keys.
    locals: HashMap<*const Expr, usize>,
}

impl Interpreter {
    fn initialize_globals(globals: &EnvironmentRef) {
        globals.borrow_mut().define(
            "clock".to_string(),
            Value::LoxCallable(Rc::new(LoxClock)),
        );
    }

    // What is "<W: Writable + 'static>" for:
    //
    // ✔️ let mut interpreter = Interpreter::new(Box::new(io::stdout()));
    // ✔️ let mut interpreter = Interpreter::new(io::stdout());
    // ✔️ let mut interpreter = Interpreter::new(Cursor::new(Vec::new()))
    // ❌ let mut interpreter = Interpreter::new(Box::new(Cursor::new(Vec::new())))
    //
    // Box::new(io::stdout()) type: Box<Stdout>, a concrete type implementing Write.
    //    
    pub fn new<W: Writable + 'static>(output: W) -> Self {
    // pub fn new(output: impl Writable + 'static) -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        Self::initialize_globals(&globals);

        let boxed_output: Box<dyn Writable> = Box::new(output);
        Interpreter { 
            output: boxed_output,
            environment: globals.clone(),
            globals: globals,
            locals: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    // Used by tests.
    pub fn get_output(&self) -> &Box<dyn Writable> {
        &self.output
    }

    #[allow(dead_code)]
    // Used by tests.
    pub fn clear_output(&mut self) {
        use std::io::Cursor;

        if let Some(cursor) = self.output.as_mut().as_any_mut().downcast_mut::<Cursor<Vec<u8>>>() {
            cursor.get_mut().clear(); // clears the Vec<u8>
            cursor.set_position(0);   // resets the write position            
        } else {
            panic!("Interpreter's output is not a mutable Cursor<Vec<u8>>");
        }
    }

    fn write_output(&mut self, value: &str) {
        writeln!(self.output, "{}", value).expect("Failed to write output");
    }

    fn evaluate(&mut self, expr: Rc<Expr>) -> Result<Value, LoxError> {
        Ok(Expr::accept_ref(expr, self)?)
    }

    fn execute(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        Stmt::accept_ref(stmt, self)
    }

    pub fn resolve(&mut self, expr: Rc<Expr>, depth: usize) {
        // Pointer identity, using pointer address: Rc::as_ptr(&expr).
        self.locals.insert(Rc::as_ptr(&expr), depth);
    }

    fn look_up_variable(&self, name: &Token, expr: Rc<Expr>) -> Result<Value, LoxError> {
        // Pointer identity, using pointer address: &Rc::as_ptr(&expr).
        if let Some(&distance) = self.locals.get(&Rc::as_ptr(&expr)) {
            Ok(Environment::get_at(&self.environment, distance, name.lexeme()))
        } else {             
            self.globals.borrow().get(name)
        }
    }

    // See: https://craftinginterpreters.com/statements-and-state.html#scope
    pub fn execute_block(&mut self, statements: &Vec<Rc<Stmt>>, 
        new_env: EnvironmentRef) -> Result<(), LoxRuntimeError> {
        let previous = std::mem::replace(&mut self.environment, new_env);

        let result = (|| {
            for stmt in statements {
                self.execute(Rc::clone(stmt))?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    // Ruby rule: false and nil are falsey, and everything else is truthy. 
    fn is_truthy(&self, object: &Value) -> bool {
        match object {
            Value::Number(_) => true,
            Value::String(_) => true,
            Value::Boolean(b) => *b,
            Value::Nil => false,
            Value::LoxCallable(_) => true,
            Value::LoxInstance(_) => true,
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        a == b
    }

    fn check_number_operand(&self, operator: &Token, 
        operand: &Value) -> Result<(), LoxError> {

        match operand {
            Value::Number(_) => Ok(()),
            _ => Err(error(operator, "Operand must be a number.")),
        }
    }

    fn expect_number(&self, operator: &Token, value: &Value) -> Result<f64, LoxError> {
        match value {
            Value::Number(n) => Ok(*n),
            _ => Err(error(operator, "Operand must be a number.")),
        }
    }

    fn binary_number_op<F>(&self, 
        operator: &Token, left: &Value, right: &Value, f: F) -> Result<Value, LoxError>
    where
        F: Fn(f64, f64) -> f64,
    {
        let l = self.expect_number(operator, left)?;
        let r = self.expect_number(operator, right)?;
        Ok(Value::Number(f(l, r)))
    }

    // Unlike the author original Java version, this version purposely 
    // keeps '.0' for f64.
    fn stringify(&self, object: &Value) -> String {
        match object {
            Value::Number(n) => format!("{:?}", n),
            Value::String(s) => s.to_string(),
            Value::Boolean(b) => b.to_string(),            
            Value::Nil => "nil".to_string(),
            Value::LoxCallable(callable) => callable.to_string(),
            Value::LoxInstance(instance) => format!("{}", instance.borrow().to_string()),
        }
    }

    // Chapter 07 version: needed for the tests.
    #[allow(dead_code)]
    pub fn interpret_single_expression(&mut self, expression: &Expr) -> Result<String, LoxError> {
        // Creating Rc<Expr> here is okay, since this method is not part of the 
        // interpreter proper. It it an integration test method and potentially 
        // a CLI method in the future.
        let expr: Rc<Expr> = Rc::new(expression.clone());
        let value: Value = self.evaluate(expr)?;

        Ok(self.stringify(&value))
    }

    // Interpret all statements, captures all errors.
    // 
    // Both evaluation ( Interpreter's successful evaluation output ) results, 
    // and evaluation errors are written to `output` in occurrence-order.
    // 
    // Evaluation errors are also captured in occurrence-order and returned 
    // via Err(LoxError). When there are multiple errors, they are separated 
    // by a newline ( \n ) character.
    pub fn interpret(&mut self, statements: &Vec<Rc<Stmt>>) -> Result<(), LoxError> {
        let mut err_msgs: Vec<String> = vec![];

        for statement in statements {
            match self.execute(Rc::clone(statement)) {
                Ok(_) => {},
                Err(err) => {
                    err_msgs.push(format!("{}", err));
                    self.write_output(&format!("{}", err));
                }
            }
        }

        if err_msgs.len() == 0 {
            Ok(())
        } else {
            Err(sys_error("", &err_msgs.join("\n")))
        }
    }
}

impl expr::Visitor<Value> for Interpreter {
    fn visit_assign_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let binding = expr.clone();
        let assign = unwrap_expr!(binding, Assign);

        let value = self.evaluate(Rc::clone(assign.value()))?;
        let result = value.clone();

        if let Some(&distance) = self.locals.get(&Rc::as_ptr(&expr)) {
            Environment::assign_at(&self.environment, distance, assign.name(), result)?;
        } else {
            self.globals.borrow_mut().assign(assign.name(), result)?;
        }

        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let binary = unwrap_expr!(expr, Binary);

        let left: Value = self.evaluate(Rc::clone(binary.left()))?;
        let right: Value = self.evaluate(Rc::clone(binary.right()))?;

        let operator: &Token = binary.operator();

        match operator.token_type() {
            TokenType::Greater => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l > r))
            }
            TokenType::GreaterEqual => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l >= r))                    
            }
            TokenType::Less => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l < r))                    
            }
            TokenType::LessEqual => {
                let l = self.expect_number(operator, &left)?;
                let r = self.expect_number(operator, &right)?;
                Ok(Value::Boolean(l <= r))                    
            }
            TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(&left, &right))),
            TokenType::Minus => Ok(self.binary_number_op(operator, &left, &right, |a, b| a - b)?),
            TokenType::Plus => {
                match (left, right) {
                    (Value::Number(ln), Value::Number(rn)) => {
                        Ok(Value::Number(ln + rn))
                    }
                    (Value::String(ls), Value::String(rs)) => {
                        Ok(Value::String(format!("{}{}", ls, rs)))
                    }                    
                    _ => Err(runtime_error(operator, 
                             "Operands must be two numbers or two strings."))
                }
            }
            TokenType::Slash => Ok(self.binary_number_op(operator, &left, &right, |a, b| a / b)?),
            TokenType::Star => Ok(self.binary_number_op(operator, &left, &right, |a, b| a * b)?),
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_call_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let call = unwrap_expr!(expr, Call);

        let callee: Value = self.evaluate(Rc::clone(call.callee()))?;
        
        let arguments: Vec<Value> = call.arguments()
            .iter()
            .map(|arg| self.evaluate(Rc::clone(arg)))
            .collect::<Result<_, _>>()?;

        match callee {
            Value::LoxCallable(func) => {
                if arguments.len() != func.arity() {
                    return Err(runtime_error(call.paren(), &format!(
                        "Expected {} arguments but got {}.", func.arity(), arguments.len())));
                }
                Ok(func.call(self, arguments)?)
            }
            _ => Err(runtime_error(call.paren(), "Can only call functions and classes."))
        }        
    }

    fn visit_get_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let get = unwrap_expr!(expr, Get);
        let object = self.evaluate(Rc::clone(get.object()))?;

        match object {
            Value::LoxInstance(instance) => 
                Ok(LoxInstance::get(Rc::clone(&instance), get.name())?),
            _ => Err(runtime_error(get.name(), "Only instances have properties."))
        }
    }

    fn visit_grouping_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let grouping = unwrap_expr!(expr, Grouping);
        Ok(self.evaluate(Rc::clone(grouping.expression()))?)
    }

    fn visit_literal_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let literal = unwrap_expr!(expr, Literal);

        match literal.value() {
            LiteralValue::Number(n) => Ok(Value::Number(*n)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::Boolean(b) => Ok(Value::Boolean(*b)),
            LiteralValue::Nil => Ok(Value::Nil),
        }
    }

    fn visit_logical_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let logical = unwrap_expr!(expr, Logical);

        let left = self.evaluate(Rc::clone(logical.left()))?;

        match logical.operator().token_type() {
            TokenType::Or if self.is_truthy(&left) => Ok(left),
            TokenType::And if !self.is_truthy(&left) => Ok(left),
            _ => Ok(self.evaluate(Rc::clone(logical.right()))?),
        }
    }

    fn visit_set_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let set = unwrap_expr!(expr, Set);

        let object = self.evaluate(Rc::clone(set.object()))?;

        match object {
            Value::LoxInstance(inst) => {
                let value = self.evaluate(Rc::clone(set.value()))?;
                inst.borrow_mut().set(set.name(), value.clone());
                Ok(value)
            },
            _ => Err(runtime_error(set.name(), "Only instances have fields."))
        }
    }

    fn visit_super_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let binding = expr.clone();
        let inner = unwrap_expr!(binding, Super);

        let distance = match self.locals.get(&Rc::as_ptr(&expr)) {
            Some(d) => *d,
            None => return Err(runtime_error(inner.method(), "Unresolved 'super' expression.")),
        };        

        let superclass = match Environment::get_at(&self.environment, distance, "super") {
            Value::LoxCallable(callable) => callable
                .as_any()
                .downcast_ref::<LoxClass>()
                .cloned()
                .ok_or_else(|| runtime_error(inner.method(), "Expecting a class."))?,
            _ => return Err(runtime_error(inner.method(), "Expecting a class.")),
        };

        let object = match Environment::get_at(&self.environment, distance - 1, "this") {
            Value::LoxInstance(instance) => instance.clone(),
            _ => return Err(runtime_error(inner.method(), "Expecting an instance.")),
        };

        match superclass.find_method(inner.method().lexeme()) {
            Some(unbound_method) => Ok(Value::LoxCallable(Rc::new(unbound_method.bind(object)))),
            None => Err(runtime_error(inner.method(), 
                &format!("Undefined property '{}'.", inner.method().lexeme()))),
        }

    }

    fn visit_this_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let binding = expr.clone();
        let this = unwrap_expr!(binding, This);

        Ok(self.look_up_variable(this.keyword(), expr)?)
    }

    fn visit_unary_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let unary = unwrap_expr!(expr, Unary);

        let right: Value = self.evaluate(Rc::clone(unary.right()))?;

        match unary.operator().token_type() {
            TokenType::Bang => Ok(Value::Boolean(!self.is_truthy(&right))),
            TokenType::Minus => {
                self.check_number_operand(unary.operator(), &right)?;
                Ok(Value::Number(-self.expect_number(unary.operator(), &right)?))
            }
            // Unreachable.
            _ => { Ok(Value::Nil) }
        }
    }

    fn visit_variable_expr(&mut self, expr: Rc<Expr>) -> Result<Value, LoxRuntimeError> {
        let binding = expr.clone();
        let variable = unwrap_expr!(binding, Variable);

        Ok(self.look_up_variable(variable.name(), expr)?)
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let block = unwrap_stmt!(stmt, Block);
        
        let new_env = Rc::new(RefCell::new(
            Environment::new_local_scope(Rc::clone(&self.environment))
        ));
        self.execute_block(block.statements(), new_env)?;

        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let class = unwrap_stmt!(stmt, Class);

        let mut superclass: Option<Rc<LoxClass>> = None;
        if let Some(expr) = class.superclass() {
            let value = self.evaluate(Rc::clone(expr))?;
            match value {
                Value::LoxCallable(callable) => {
                    if let Some(lox_class) = callable.as_any().downcast_ref::<LoxClass>() {
                        superclass = Some(Rc::new(lox_class.clone()));
                    } else {
                        return Err(runtime_error(class.name(), "Superclass must be a class."));
                    }
                }
                _ => return Err(runtime_error(class.name(), "Superclass must be a class.")),
            }
        }

        self.environment.borrow_mut().define(class.name().lexeme().to_string(), Value::Nil); 

        let enclosing = Rc::clone(&self.environment);

        if let Some(expr) = class.superclass() {
            let value = self.evaluate(Rc::clone(expr))?;
            let super_env = Rc::new(RefCell::new(Environment::new_local_scope(Rc::clone(&self.environment))));
            super_env.borrow_mut().define("super".to_string(), value);
            self.environment = Rc::clone(&super_env);
        }

        let mut methods: LoxFunctionsMap = HashMap::new();
        for method in class.methods() {
            let function: LoxFunction = LoxFunction::new(method.as_ref().clone(), 
                self.environment.clone(), method.name().lexeme() == "init");
            methods.insert(method.name().lexeme().to_string(), function);
        }

        let klass: LoxClass = LoxClass::new(class.name().lexeme().to_string(), 
            superclass, methods);

        if class.superclass().is_some() {
            self.environment = enclosing;
        }

        self.environment.borrow_mut().assign(
            class.name(),
            Value::LoxCallable(Rc::new(klass))
        )?;        
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let expression = unwrap_stmt!(stmt, Expression);
        self.evaluate(Rc::clone(expression.expression()))?;

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, Function);

        let function: LoxFunction = LoxFunction::new(inner.clone(), Rc::clone(&self.environment), false);
        self.environment.borrow_mut().define(
            inner.name().lexeme().to_string(), 
            Value::LoxCallable(Rc::new(function))
        );
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, If);

        let value: Value = self.evaluate(Rc::clone(inner.condition()))?;
        if self.is_truthy(&value) {
            Ok(self.execute(Rc::clone(inner.then_branch()))?)
        } else if let Some(else_branch) = inner.else_branch() {
            Ok(self.execute(Rc::clone(else_branch))?)
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let print = unwrap_stmt!(stmt, Print);

        let value: Value = self.evaluate(Rc::clone(print.expression()))?;

        // Note from the author in the original Java version:
        //     Before discarding the expression’s value, we convert it to a 
        //     string using the stringify() method we introduced in the last 
        //     chapter and then dump it to stdout.
        self.write_output(&self.stringify(&value));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, Return);

        let value = if let Some(expr) = &inner.value() {
            self.evaluate(Rc::clone(expr))?
        } else {
            Value::Nil
        };

        Err(LoxRuntimeError::Return(LoxReturn { value }))
    }

    fn visit_var_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let var = unwrap_stmt!(stmt, Var);
        let value = if let Some(initializer) = var.initializer() {
            self.evaluate(Rc::clone(initializer))?
        } else {
            Value::Nil
        };

        self.environment.borrow_mut().define(var.name().lexeme().to_string(), value);
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: Rc<Stmt>) -> Result<(), LoxRuntimeError> {
        let inner = unwrap_stmt!(stmt, While);

        let mut value: Value = self.evaluate(Rc::clone(inner.condition()))?;
        while self.is_truthy(&value) {
            self.execute(Rc::clone(inner.body()))?;
            value = self.evaluate(Rc::clone(inner.condition()))?;
        }
        Ok(())
    }
}

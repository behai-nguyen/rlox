/* Date Created: 15/07/2025. */

//! The **Environments** section in  
//! [https://craftinginterpreters.com/statements-and-state.html#environments](https://craftinginterpreters.com/statements-and-state.html#environments).
//! 

// To run test for this module only: 
// 
//     * cargo test environment::tests
//
//     * cargo test environment::tests::define_and_retrieve_number -- --exact [--nocapture]
//     * cargo test environment::tests::retrieve_missing_variable -- --exact [--nocapture]
//     * cargo test environment::tests::overwrite_and_retrieve_string -- --exact [--nocapture]
//     * cargo test environment::tests::assign_to_existing_variable -- --exact [--nocapture]
//     * cargo test environment::tests::assign_to_non_existing_variable -- --exact [--nocapture]
//     * cargo test environment::tests::parent_pointer_tree -- --exact [--nocapture]
// 

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::lox_error::LoxError;
use super::lox_error_helper::error; 
use super::data_type::Value;
use super::token::Token;

type ValuesMap = HashMap<String, Value>;

pub type EnvironmentRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: ValuesMap,
    enclosing: Option<EnvironmentRef>,
}

impl Environment {
    // https://craftinginterpreters.com/statements-and-state.html#nesting-and-shadowing
    // The global scope’s environment.
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    // https://craftinginterpreters.com/statements-and-state.html#nesting-and-shadowing
    // Creates a new local scope nested inside the given outer one.
    pub fn new_local_scope(enclosing: EnvironmentRef) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, LoxError> {
        if let Some(token) = self.values.get(name.lexeme()) {
            Ok(token.clone())
        } else {
            if let Some(env) = &self.enclosing {
                return env.borrow().get(name);
            }
            Err(error(name, &format!("Undefined variable '{}'.", name.lexeme())))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), LoxError> {
        match self.values.get_mut(name.lexeme()) {
            Some(val) => {
                *val = value;
                Ok(())
            }
            None => {
                if let Some(env) = &mut self.enclosing {
                    env.borrow_mut().assign(name, value)
                } else {
                    Err(error(name, &format!("Undefined variable '{}'.", name.lexeme())))
                }
            }
        }
    }
    
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn ancestor(env: &EnvironmentRef, name: &Token, distance: usize) -> Result<EnvironmentRef, LoxError> {
        let mut environment = Rc::clone(env);

        for _ in 0..distance {
            let next = {
                let borrowed = environment.borrow();
                borrowed.enclosing.clone()
            };

            environment = next.ok_or_else(|| error(name, "No enclosing environment found."))?;
        }

        Ok(environment)
    }

    pub fn get_at(env: &EnvironmentRef, name: &Token, distance: usize) -> Result<Value, LoxError> {
        Self::ancestor(env, name, distance)?.borrow().get(name)
    }

    pub fn assign_at(env: &EnvironmentRef, distance: usize, 
        name: &Token, value: Value) -> Result<(), LoxError> {
        Self::ancestor(env, name, distance)?.borrow_mut().values.insert(
            name.lexeme().to_string(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::data_type::Value;
    use crate::environment::Environment;
    use crate::token_type::TokenType;
    use crate::token::Token;

    fn retrieve_as_number(env: &Environment, token: &Token) -> f64 {
        let res = env.get(token);
        assert!(res.is_ok());
        if let Value::Number(n) = res.unwrap() {
            n
        } else {
            panic!("Expected a number!");
        }        
    }

    fn retrieve_as_string(env: &Environment, token: &Token) -> String {
        let res = env.get(token);
        assert!(res.is_ok());
        if let Value::String(s) = res.unwrap() {
            s
        } else {
            panic!("Expected a number!");
        }        
    }

    #[test]
    fn define_and_retrieve_number() {
        let mut environment = Environment::new();

        environment.define("scale".to_string(), Value::Number(4.91));

        let token = Token::new(TokenType::Var, "scale".to_string(), None, 1);
        let res = environment.get(&token);
        assert!(res.is_ok());
        if let Value::Number(n) = res.unwrap() {
            assert_eq!(n, 4.91);
        } else {
            assert_eq!(true, false, "Expected a number!");
        }
    }

    #[test]
    fn retrieve_missing_variable() {
        let mut environment = Environment::new();

        environment.define("scale".to_string(), Value::Number(4.91));

        let token = Token::new(TokenType::Var, "str".to_string(), None, 10);
        let res = environment.get(&token);
        assert_eq!(true, res.is_err());
        assert_eq!("[line 10] Error at 'str': Undefined variable 'str'.", res.unwrap_err().to_string());
    }

    #[test]
    fn overwrite_and_retrieve_string() {
        let mut environment = Environment::new();

        environment.define("scale".to_string(), Value::Number(4.91));

        environment.define("scale".to_string(), Value::String("double".to_string()));
        environment.define("scale_factor".to_string(), Value::Number(4.91));
        // Retrieves a String variable.
        let token = Token::new(TokenType::Var, "scale".to_string(), None, 1);
        let res = environment.get(&token);
        assert!(res.is_ok());
        if let Value::String(s) = res.unwrap() {
            assert_eq!(s, "double");
        } else {
            assert_eq!(true, false, "Expected a string of value 'double'!");
        }
    }

    #[test]
    fn assign_to_existing_variable() {
        let mut environment = Environment::new();

        environment.define("scale".to_string(), Value::Number(4.91));
        let token = Token::new(TokenType::Var, "scale".to_string(), None, 1);

        // Assert the original value.
        let n = retrieve_as_number(&environment, &token);
        assert_eq!(n, 4.91);

        // Test 1: overwrites value with same type.
        let res = environment.assign(&token, Value::Number(15.01));
        assert!(res.is_ok());
        // Assert the overwritten value.
        let n = retrieve_as_number(&environment, &token);
        assert_eq!(n, 15.01);

        // Test 2: overwrites value with a different type.
        let res = environment.assign(&token, Value::String("double".to_string()));
        assert!(res.is_ok());
        // Assert the overwritten value.
        let s = retrieve_as_string(&environment, &token);
        assert_eq!(s, "double");
    }

    #[test]
    fn assign_to_non_existing_variable() {
        let mut environment = Environment::new();

        environment.define("scale".to_string(), Value::Number(4.91));

        // Token with a diffrent name: Non-existing variable.
        let token = Token::new(TokenType::Var, "scale_factor".to_string(), None, 8);

        // Test: overwrites a non-existing variable.
        let res = environment.assign(&token, Value::Number(15.01));
        assert!(res.is_err());
        assert_eq!("[line 8] Error at 'scale_factor': Undefined variable 'scale_factor'.", res.unwrap_err().to_string());
    }

    #[test]
    fn parent_pointer_tree() {
        // Creates the outer-most environment, and add a single variable to it.
        let global_env= Rc::new(RefCell::new(Environment::new()));
        global_env.borrow_mut().define("scale".to_string(), Value::Number(4.91));

        // Creates a scope environment, chains it to the outer-most one. Also add 
        // a single variable to it.
        let mut scope_env = Environment::new_local_scope(Rc::clone(&global_env));
        scope_env.define("scale_factor".to_string(), Value::String("double".to_string()));

        // Creates a Token to attempt to retrieve a variable.
        let token = Token::new(TokenType::Var, "scale".to_string(), None, 3);

        // Test: from the scope environment, attempt to retrieve a variable in the 
        //     outer-most environment: Expected to succeed.
        let n = retrieve_as_number(&scope_env, &token);
        assert_eq!(n, 4.91);

        // Creates a different Token to attempt to retrieve a different variable.
        let token = Token::new(TokenType::Var, "scale_factor".to_string(), None, 7);

        // Test: from the scope environment, attempt to retrieve a variable in 
        //     itself: Expected to succeed.
        let s = retrieve_as_string(&scope_env, &token);
        assert_eq!(s, "double".to_string());

        // Creates a Token to attempt to retrieve a variable.
        let token = Token::new(TokenType::Var, "scale".to_string(), None, 10);

        // Test: a variable defined only in the global scope is NOT found in the 
        //     local scope if shadowed.
        scope_env.define("scale".to_string(), Value::Number(99.0));
        let n = retrieve_as_number(&scope_env, &token);
        assert_eq!(n, 99.0); // Confirms shadowing

        // Test: confirming that global scope still has a variable whose name is 
        //     the same as the shadowed one, but the values are different.
        let n = retrieve_as_number(&global_env.borrow(), &token);
        assert_eq!(n, 4.91);
    }
}
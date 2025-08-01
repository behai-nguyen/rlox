/* Date Created: 28/05/2025. */

//! The **The Scanner Class** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 

use std::env;
use std::process;
use std::fs::read_to_string;
use std::io::{self, Write};

mod lox_error;
mod lox_error_helper;
mod scanner_index;
mod token_type;
mod token;
mod data_type;
mod lox_callable;
mod lox_clock;
mod scanner;
mod ast_printer;
mod parser;
mod environment;
mod interpreter;
mod lox_function;
mod lox_return;
mod lox_runtime_error;

mod expr;
mod stmt;

use scanner::Scanner;
use interpreter::Interpreter;

fn run(source: &str) -> Result<(), std::io::Error> {
    let mut scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(&tokens);
            match parser.parse() {
                Err(err) => println!("Parsing error: {}", err),
                Ok(statements) => { 
                    // Both are valid.
                    // let mut interpreter = Interpreter::new(Box::new(io::stdout()));
                    let mut interpreter = Interpreter::new(io::stdout());
                    match interpreter.interpret(statements) {
                        Err(err) => println!("Evaluation error: {}", err),
                        Ok(_) => (),
                    }
                },
            }
        },
        Err(err) => {
            println!("{}", err);
            process::exit(65);
        }
    }

    Ok(())
}

pub fn run_file(scriptfile: &str) -> Result<(), std::io::Error> {
    let contents = read_to_string(scriptfile)?;
    run(&contents)?;

    Ok(())
}

pub fn run_prompt() -> Result<(), std::io::Error> {
    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        io::stdin()
            .read_line(&mut line)?;

        // Remove \r\n
        line.retain(|c| c != '\r' && c != '\n');

        if line.len() == 0 {
            break;
        }

        run(&line)?;

        // Empty the string.
        line.clear();
    }

    Ok(())
}

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: {} [script]", &args[0]);
        process::exit(1);
    } else if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        let _ = run_prompt();
    }
}
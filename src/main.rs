/* Date Created: 28/05/2025. */

//! The **The Scanner Class** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 

use std::env;
use std::process;
use std::fs::{read_to_string, exists};
use std::io::{self, Write};

mod expr;
mod stmt;
mod lox_error;
mod lox_error_helper;
mod scanner_index;
mod token_type;
mod token;
mod value;
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
mod resolver;
mod lox_class;
mod lox_instance;

use rlox::{unwrap_expr, unwrap_stmt};

use scanner::Scanner;
use interpreter::Interpreter;
use resolver::Resolver;
use lox_error::LoxError;

fn print_error(err: LoxError, originator: &str) {
    println!("{} error:", originator);

    let err_msgs: Vec<String> = err
        .to_string()
        .lines()
        .map(|line| line.to_string())
        .collect();
    for err_msg in err_msgs {
        println!("    {}", err_msg);
    }
}

fn run(source: &str) -> Result<(), std::io::Error> {
    let mut scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Err(err) => print_error(err, "Scanner"),
        Ok(tokens) => {
            let mut parser = parser::Parser::new(&tokens);
            match parser.parse() {
                Err(err) => print_error(err, "Parser"),
                Ok(statements) => { 
                    // Both are valid.
                    // let mut interpreter = Interpreter::new(Box::new(io::stdout()));
                    let mut interpreter = Interpreter::new(io::stdout());
                    let mut resolver: Resolver = Resolver::new(&mut interpreter);

                    match resolver.resolve(&statements) {
                        Err(err) => print_error(err.into(), "Resolver"),
                        Ok(_) => {
                            match interpreter.interpret(&statements) {
                                Err(err) => print_error(err.into(), "Interpreter"),
                                Ok(_) => (),
                            }
                        }
                    }
                },
            }
        },
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
        if !exists(&args[1]).expect("Can not check if input file exists.") {
            println!("Input file `{}` does not exist!", &args[1]);
            process::exit(65);
        }

        let _ = run_file(&args[1]);
    } else {
        let _ = run_prompt();
    }
}
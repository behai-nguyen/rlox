/* Date Created: 28/05/2025. */

//! The **The Scanner Class** in  
//! [https://craftinginterpreters.com/scanning.html](https://craftinginterpreters.com/scanning.html). 

use std::env;
use std::process;
use std::fs::read_to_string;
use std::io::{self, Write};

mod lox_error;
mod scanner_index;
mod token_type;
mod token;
mod scanner;
mod ast_printer;
mod parser;
mod interpreter;

mod expr;
mod stmt;

use scanner::Scanner;
use ast_printer::AstPrinter;
use interpreter::Interpreter;

fn report(line: usize, err_where: &str, message: &str) {
    println!("[line {line}] Error {err_where}: {message}");
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn run(source: &str) -> Result<(), std::io::Error> {
    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => {
            let parser = parser::Parser::new(&tokens);
            match parser.parse() {
                Err(err) => println!("Parsing error: {}", err),
                Ok(expr) => { 
                    println!("Expression: {}", AstPrinter{}.print_expression(&expr).unwrap()); 

                    let interpreter = Interpreter{};
                    match interpreter.interpret(&expr) {
                        Err(err) => println!("Evaluation error: {}", err),
                        Ok(val) => println!("Evaluated to: {}", val),
                    }
                },
            }
        },
        Err(err) => {
            error(err.get_line(), &err.get_err_msg());
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
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

use scanner::Scanner;

fn report(line: usize, err_where: &str, message: &str) {
    println!("[line {line}] Error {err_where}: {message}");
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn run(source: &str) -> Result<(), std::io::Error> {
    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => {
            for token in tokens {
                println!("Token: {token}")
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
        line.retain(|c| (c != '\r') & (c != '\n' ));

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
        println!("Usage: jlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        let _ = run_prompt();
    }
}

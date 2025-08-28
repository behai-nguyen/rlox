/* Date Created: 14/06/2025. */

//! The **Representing Code** in  
//! [https://craftinginterpreters.com/representing-code.html](https://craftinginterpreters.com/representing-code.html)
//! 
//! The full original Java version can be found at:
//! https://github.com/munificent/craftinginterpreters/blob/master/java/com/craftinginterpreters/tool/GenerateAst.java

use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct GenerateAst;

impl GenerateAst {
    // Rust-specific: not in the original Java version.
    fn get_field_metadata(field: &str) -> (&str, &str) {
        // Field is each of: "Expr left", "Token operator" and "Expr right".
        let last_space = field.rfind(' ').unwrap();
        let (type_part_raw, name_part_raw) = field.split_at(last_space);

        (type_part_raw.trim(), name_part_raw.trim())
    }

    fn accept_pattern_matching_dispatch(file: &mut File, 
        base_name: &str,
        types: &Vec<&str>
    ) -> Result<(), io::Error> {
        for t in types {
            // t: "Assign   : Token name, Rc<Expr> value"
            let last_colon = t.rfind(':').unwrap();
            let (type_name, _) = t.split_at(last_colon);
            let trimmed_type = type_name.trim();

            file.write_all(format!("            {0}::{1}(_) => visitor.visit_{2}_{3}({3}),\n",
                base_name, trimmed_type, trimmed_type.to_lowercase(), 
                base_name.to_lowercase()).as_bytes()
            )?;
        }

        Ok(())
    }

    fn define_visitor(file: &mut File,
        base_name: &str,
        types: &Vec<&str>
    ) -> Result<(), io::Error> {
        file.write_all("// Define enum\n".as_bytes())?;
        file.write_all("#[derive(Debug, Clone, Hash, Eq, PartialEq)]\n".as_bytes())?;
        file.write_all(format!("pub enum {} {{\n", base_name).as_bytes())?;
        for t in types {
            // t: "Assign   : Token name, Box<Expr> value"
            let last_colon = t.rfind(':').unwrap();
            let (type_name, _) = t.split_at(last_colon);
            file.write_all(format!("    {0}({0}),\n", type_name.trim()).as_bytes())?;
        }
        file.write_all("}\n\n".as_bytes())?;

        file.write_all("// Visitor Trait\n".as_bytes())?;
        file.write_all("pub trait Visitor<T> {\n".as_bytes())?;
        for t in types {
            // t: "Assign   : Token name, Box<Expr> value"
            let last_colon = t.rfind(':').unwrap();
            let (type_name, _) = t.split_at(last_colon);
            let trimmed_type = type_name.trim();

            file.write_all(format!("    fn visit_{0}_{1}(&mut self, {1}: Rc<{2}>) -> Result<T, LoxRuntimeError>;\n",
                trimmed_type.to_lowercase(), base_name.to_lowercase(), base_name).as_bytes()
            )?;
        }
        file.write_all("}\n\n".as_bytes())?;

        // file.write_all(format!("// Implement `accept()`, `accept_ref()` for `{}`\n", base_name).as_bytes())?;
        file.write_all(format!("// Implement `accept()` for `{}`.\n", base_name).as_bytes())?;
        file.write_all(format!("impl {} {{\n", base_name).as_bytes())?;
        file.write_all(format!("    pub fn accept<T>({0}: Rc<{1}>, visitor: &mut dyn Visitor<T>) \
                                        -> Result<T, LoxRuntimeError> {{\n", base_name.to_lowercase(), base_name).as_bytes())?;        
        file.write_all(format!("        match {}.as_ref() {{\n", base_name.to_lowercase()).as_bytes())?;
        Self::accept_pattern_matching_dispatch(file, base_name, types)?;
        file.write_all("        }\n".as_bytes())?; // match self closing
        file.write_all("    }\n".as_bytes())?; // pub fn accept() closing.

        file.write_all("}\n".as_bytes())?;

        Ok(())
    }

    fn define_type(file: &mut File,
        class_name: &str,
        field_list: &str
    ) -> Result<(), io::Error> {

        // Expr left, Token operator, Expr right
        let fields: Vec<&str> = field_list.split(",").map(|f| f.trim()).collect();

        // struct definition.
        file.write_all("#[derive(Debug, Clone, Hash, Eq, PartialEq)]\n".as_bytes())?;
        file.write_all(format!("pub struct {} {{\n", class_name).as_bytes())?;
        for field in &fields {
            // Each of: "Box<Expr> left", "Token operator", "Box<Expr> right", etc.
            let (type_part, name_part) = Self::get_field_metadata(field);

            file.write_all(format!("    {}: {},\n", name_part, type_part).as_bytes())?;
        }
        file.write_all("}\n\n".as_bytes())?;

        // struct impl.
        file.write_all(format!("impl {} {{\n", class_name).as_bytes())?;
        file.write_all("    pub fn new(".as_bytes())?;
        for (i, field) in fields.iter().enumerate() {
            // Each of: "Expr left", "Token operator", "Expr right", etc.
            let (type_part, name_part) = Self::get_field_metadata(field);

            if i == 0 {
                file.write_all(format!("{}: {}", name_part, type_part).as_bytes())?;
            } else {
                file.write_all(format!(", \n        {}: {}", name_part, type_part).as_bytes())?;
            }

            if (i + 1) == fields.len() {
                if i == 0 {
                    file.write_all(") -> Self {\n".as_bytes())?;
                } else {
                    file.write_all("\n    ) -> Self {\n".as_bytes())?;
                }
            }
        }
        // Constructing Self
        file.write_all(format!("        {} {{\n", class_name).as_bytes())?;
        for field in &fields {
            // Each of: "Expr left", "Token operator", "Expr right", etc.
            let (_type_part, name_part) = Self::get_field_metadata(field);
            file.write_all(format!("            {},\n", name_part).as_bytes())?;
        }
        // Constructing Self
        file.write_all("        }\n".as_bytes())?;
        // pub fn new()
        file.write_all("    }\n\n".as_bytes())?;

        for field in fields {
            // Each of: "Expr left", "Token operator", "Expr right", etc.
            let (type_part, name_part) = Self::get_field_metadata(field);

            file.write_all(format!("    pub fn {}(&self) -> &{} {{\n",
                name_part, type_part).as_bytes())?;
            file.write_all(format!("        &self.{}\n", name_part).as_bytes())?;
            file.write_all("    }\n\n".as_bytes())?;
        }

        // Closing impl block.
        file.write_all("}\n\n".as_bytes())?;

        Ok(())
    }

    fn define_ast(own_contents: Vec<&str>,
        base_name: &str,
        output_dir: &str,
        types: Vec<&str>
    ) -> Result<(), io::Error> {
        let file_name = Path::new(output_dir).join(format!("{}.rs", base_name.to_lowercase()));
        let mut file = File::create(&file_name)?;

        file.write_all(format!("/// Appendix II {}\n", base_name.to_lowercase()).as_bytes())?;

        for content in own_contents {
            file.write_all(format!("{}", content).as_bytes())?;
        }

        for t in &types {
            let type_desc: Vec<&str> = t.split(":").collect();
            let class_name = type_desc[0].trim();
            let fields = type_desc[1].trim();
            Self::define_type(&mut file, class_name, fields)?;
        }

        Self::define_visitor(&mut file, base_name, &types)?;

        file.write_all(format!("//< Appendix II {}\n", base_name.to_lowercase()).as_bytes())?;

        println!("{:?} module has been successfully created.", file_name);

        Ok(())
    }
}

fn main() {
    // Collect command line arguments.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <output directory>", &args[0]);
        process::exit(1);
    }

    // Attempt to prepare output directories.
    match fs::create_dir_all(&args[1]) {
        Ok(_val) => println!("Directories created."),
        Err(e) => {
            println!("Failed to create directories: {:?}", e);
            process::exit(1);
        }
    }

    let _ = GenerateAst::define_ast(vec!["use std::rc::Rc;\n\n",
                "use super::token::{LiteralValue, Token};\n", 
	            "use super::lox_runtime_error::LoxRuntimeError;\n\n"], 
                "Expr", &args[1], vec![
				    "Assign   : Token name, Rc<Expr> value",
                    "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right",
                    "Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments",
                    "Get      : Rc<Expr> object, Token name",
                    "Grouping : Rc<Expr> expression",
                    "Literal  : LiteralValue value",
                    "Logical  : Rc<Expr> left, Token operator, Rc<Expr> right",
                    "Set      : Rc<Expr> object, Token name, Rc<Expr> value",
                    "Super    : Token keyword, Token method",
                    "This     : Token keyword",
                    "Unary    : Token operator, Rc<Expr> right",
                    "Variable : Token name"
				]);

    let _ = GenerateAst::define_ast(vec!["use std::rc::Rc;\n\n",
                "use super::token::Token;\n",
                "use super::expr::Expr;\n",
				"use super::lox_runtime_error::LoxRuntimeError;\n\n"], 
                "Stmt", &args[1], vec![
                    "Block      : Vec<Rc<Stmt>> statements",
                    "Class      : Token name, Option<Rc<Expr>> superclass, \
                                  Vec<Rc<Function>> methods",
                    "Expression : Rc<Expr> expression",
                    "Function   : Token name, Vec<Token> params, \
                                  Vec<Rc<Stmt>> body",
                    "If         : Rc<Expr> condition, Rc<Stmt> then_branch, \
                                  Option<Rc<Stmt>> else_branch",
                    "Print      : Rc<Expr> expression",
                    "Return     : Token keyword, Option<Rc<Expr>> value",
                    "Var        : Token name, Option<Rc<Expr>> initializer",
                    "While      : Rc<Expr> condition, Rc<Stmt> body"
				]);
}

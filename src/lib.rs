use std::borrow::Borrow;
use std::fs;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::Path;

use failure::Fail;

pub(crate) mod ast;
// pub(crate) mod ast_rewrite;
pub mod error;
pub(crate) mod interpreter;
pub(crate) mod parser;
pub(crate) mod token;

use crate::ast::{printer::Printer, visit::Visitor, visit::*, Program};
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::parser::LoxParser;

/// A Lox program.
pub struct Lox;

impl Lox {
    /// Run Lox code in a file
    pub fn run_file<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let contents = fs::read_to_string(path)?;
        Lox::run(contents)
    }

    /// Run any utf-8 str of Lox code
    pub fn run<C: Borrow<str>>(input: C) -> Result<(), Error> {
        let code = input.borrow();

        let pairs = LoxParser::parse_str(code)
            .map_err(|e| eprintln!("{:#?}", e))
            .unwrap();

        // let ast = Ast::from_program(pairs);
        let mut ast = Program::from_pairs(pairs);

        // println!("{:#?}", ast);
        let mut printer = Printer(0);
        let mut interpreter = Interpreter;
        ast.visit(&mut printer)?;
        println!("=== Execution ===");
        match ast.visit(&mut interpreter) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }

        // ast.pretty_print(0.into(), 0);

        Ok(())
    }

    /// Start the Lox REPL
    pub fn run_prompt() -> Result<(), Error> {
        let stdin = stdin();
        let mut lines = stdin.lock().lines();

        print!("> ");
        stdout().flush()?;

        while let Some(line) = lines.next().transpose()? {
            Lox::run(line)?;
            
            print!("> ");
            stdout().flush()?;
        }

        Ok(())
    }
}

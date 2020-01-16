#![feature(box_patterns, slice_patterns)]

use std::borrow::Borrow;
use std::fs;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::{Path, PathBuf};

use structopt::StructOpt;

pub(crate) mod ast;
// pub(crate) mod ast_rewrite;
pub mod error;
pub(crate) mod interpreter;
pub(crate) mod parser;
pub(crate) mod token;
// pub(crate) mod visitor;
pub(crate) mod env;
// pub mod

use crate::ast::{printer::Printer, visit::*, Program};
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::parser::LoxParser;

/// A Lox program.
pub struct Lox;

impl Lox {
    /// Run Lox code in a file
    pub fn run_file<P: AsRef<Path>>(path: P, config: &Config) -> Result<(), Error> {
        let contents = fs::read_to_string(path)?;
        let mut interpreter = Interpreter::new();
        Lox::run(contents, config, &mut interpreter)
    }

    /// Run any utf-8 str of Lox code
    pub fn run<C: Borrow<str>>(input: C, config: &Config, interpreter: &mut Interpreter) -> Result<(), Error> {
        let code = input.borrow();

        let pairs = LoxParser::parse_str(code)
            .map_err(|e| eprintln!("{:#?}", e))
            .unwrap();

        if config.parse_tree {
            println!("{:#?}", pairs);
        }

        let mut ast = Program::from_pairs(pairs);
        
        if config.emit_ast {
            let mut printer = Printer(0);
            printer.visit_program(&mut ast)?;
        }

        match interpreter.visit_program(&mut ast) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }

        Ok(())
    }

    /// Start the Lox REPL
    pub fn run_prompt(config: &Config) -> Result<(), Error> {
        let stdin = stdin();
        let mut lines = stdin.lock().lines();
        let mut interpreter = Interpreter::new();

        print!("> ");
        stdout().flush()?;

        while let Some(line) = lines.next().transpose()? {
            Lox::run(line, config, &mut interpreter)?;

            print!("> ");
            stdout().flush()?;
        }

        Ok(())
    }
}

/// A Rust interpreter for the Lox programming language.
#[derive(Debug, Default, Clone, StructOpt)]
#[structopt(name = "lox")]
pub struct Config {
    /// The path of the .lox file to execute
    #[structopt(short = "f", long = "file")]
    pub path: Option<PathBuf>,
    /// Print the Parse Tree
    #[structopt(short = "p", long = "parse-tree")]
    pub parse_tree: bool,
    /// Print the HIR AST
    #[structopt(short = "a", long = "emit-ast")]
    pub emit_ast: bool,
}
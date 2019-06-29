use std::io::{stdin, BufRead, stdout, Write};
use std::path::Path;
use std::borrow::Borrow;
use std::fs;

use crate::error::Error;

pub mod error;

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
        
        unimplemented!()
    }

    /// Start the Lox REPL
    pub fn run_prompt() -> Result<(), Error> {
        let stdin = stdin();
        let mut lines = stdin.lock().lines();

        print!("> ");
        stdout().flush()?;

        while let Some(line) = lines.next().transpose()? {
            print!("> ");
            stdout().flush()?;

            Lox::run(line)?;
        }

        Ok(())
    }
}

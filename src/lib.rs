use std::io::{stdin, BufRead, stdout, Write};
use std::path::Path;
use std::borrow::Borrow;
use std::fs;

/// A Lox program.
pub struct Lox;

impl Lox {
    /// Run Lox code in a file
    pub fn run_file<P: AsRef<Path>>(path: P) {
        let contents = fs::read_to_string(path).unwrap(); 
        Lox::run(contents);
    }

    /// Run any utf-8 str of Lox code 
    pub fn run<C: Borrow<str>>(input: C) {
        let code = input.borrow();
        // todo...
    }

    /// Start the Lox REPL
    pub fn run_prompt() {
        let stdin = stdin();
        let mut lines = stdin.lock().lines();

        print!("> ");
        stdout().flush().unwrap();

        while let Some(line) = lines.next().transpose().unwrap() {
            print!("> ");
            stdout().flush().unwrap();

            Lox::run(line);
        }
    }
}

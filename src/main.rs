use lox::Lox;

use std::path::PathBuf;
use structopt::StructOpt;

/// A Rust interpreter for the Lox programming language. 
#[derive(Debug, Default, Clone, StructOpt)]
#[structopt(name = "lox")]
pub struct Config {
    /// The path of the .lox file to execute 
    #[structopt(short = "f", long = "file")]
    path: Option<PathBuf>,
}

fn main() {
    let args = Config::from_args();
    
    if let Some(path) = args.path {
        Lox::run_file(path)
    } else {
        Lox::run_prompt()
    }
}
use lox::Lox;
use lox::Config;

use std::path::PathBuf;
use structopt::StructOpt;
pub mod parser;

fn main() {
    let args = Config::from_args();

    let result = if let Some(ref path) = &args.path {
        Lox::run_file(path, &args)
    } else {
        Lox::run_prompt(&args)
    };

    result.map_err(|e| eprintln!("{}", e)).unwrap();
}

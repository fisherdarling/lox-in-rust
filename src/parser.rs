use pest::{error::Error, iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Debug, Copy, Clone, Parser)]
#[grammar = "lox.pest"]
pub struct LoxParser;

impl LoxParser {
    pub fn parse_str(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        LoxParser::parse(Rule::program, input)
    }
}

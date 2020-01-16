use crate::ast::operator::{BinOp, UnOp};
use crate::ast::Ident;
use failure::Fail;
use std::io::Error as IOError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "An IO Error was encountered: {:?}", 0)]
    IOError(#[cause] IOError),
    #[fail(display = "Type Mismatch: attempted to convert `{}` into `{}`", 0, 1)]
    TypeMismatch(String, String),
    #[fail(display = "Invalid Operator: {} {} {}", 0, 1, 2)]
    InvalidBinaryOperator(String, BinOp, String),
    #[fail(display = "Invalid Operator: {} {}", 0, 1)]
    InvalidUnaryOperator(UnOp, String),
    #[fail(display = "Expected Value")]
    ExpectedValue,
    #[fail(display = "Undefined variable `{}`", 0)]
    UndefinedVariable(Ident),
    #[fail(display = "Unsupported Operation `{}`", 0)]
    UnsupportedOperation(String),
    #[fail(display = "Unsupported Truthiness `{}`", 0)]
    UnsupportedTruthiness(String),
    #[fail(display = "Invalid number of arguments, expected `{}` arg(s), got `{}` arg(s)", 0, 1)]
    ArgumentArity(usize, usize),
    
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error::IOError(error)
    }
}

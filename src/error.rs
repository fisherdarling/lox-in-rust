use crate::ast::operator::{BinOp, UnOp};
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
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error::IOError(error)
    }
}

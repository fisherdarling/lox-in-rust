use failure::Fail;
use std::io::Error as IOError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "An IO Error was encountered: {:?}", 0)]
    IOError(#[cause] IOError)
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error::IOError(error)
    }
}
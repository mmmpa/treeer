use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TreeerErr {
    errors: Errors
}

impl Default for TreeerErr { fn default() -> Self { Self { errors: Errors::Unknown } } }
impl Display for TreeerErr { fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { write!(f, "{:?}", self) } }
impl Error for TreeerErr {}

#[derive(Debug)]
pub enum Errors {
    Unknown,
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl From<()> for TreeerErr {
    fn from(_: ()) -> Self {
        Self { errors: Errors::Unknown }
    }
}

impl From<std::io::Error> for TreeerErr {
    fn from(e: std::io::Error) -> Self {
        Self { errors: Errors::IoError(e) }
    }
}

impl From<std::string::FromUtf8Error> for TreeerErr {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self { errors: Errors::FromUtf8Error(e) }
    }
}

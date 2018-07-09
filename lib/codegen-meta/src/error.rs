use std::fmt;
use std::io;

/// An error that occurred when the cretonne_codegen_meta crate was generating
/// files for the cretonne_codegen crate.
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
}

impl Error {
    /// Create a new error object with the given message.
    pub fn with_msg<S: Into<String>>(msg: S) -> Error {
        Error {
            inner: Box::new(ErrorInner::Msg(msg.into())),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<io::Error> for Error {
    fn from(_e: io::Error) -> Self {
        unimplemented!();
    }
}

#[derive(Debug)]
pub enum ErrorInner {
    Msg(String),
    IoError(io::Error),
}

impl fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorInner::Msg(s) => write!(f, "{}", s),
            ErrorInner::IoError(e) => write!(f, "{}", e),
        }
    }
}

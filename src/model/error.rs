use std::error;
use std::fmt;
use serde::{Serialize, Deserialize};

/// todo: docs ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    /// todo: docs ...
    code: i32,
    /// todo: docs ...
    message: String,
}

impl fmt::Display for Error {
    /// todo: docs ...
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", self.code, self.message)
    }
}

impl error::Error for Error {
    /// todo: docs ...
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub(crate) enum RloxError {
    IoError(std::io::Error),
    SyntaxError(RloxSyntaxError),
}

#[derive(Debug)]
pub(crate) struct RloxSyntaxError {
    pub(crate) line_number: usize,
    pub(crate) description: String,
}

impl Display for RloxSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rlox syntax error: \nline_number: {}\n, description: {}\n",
            self.line_number, self.description
        )
    }
}

impl From<std::io::Error> for RloxError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<RloxSyntaxError> for RloxError {
    fn from(e: RloxSyntaxError) -> Self {
        Self::SyntaxError(e)
    }
}

impl Display for RloxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RloxError::*;
        match self {
            IoError(e) => write!(f, "error reading script: {}", e),
            SyntaxError(e) => write!(f, "Syntax error: {}", e),
        }
    }
}

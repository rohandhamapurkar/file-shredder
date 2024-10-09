use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    NotEnoughArgumentsErr,
    PathNonExistErr,
    InvalidPassesErr,
    InvalidThreadCountErr
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotEnoughArgumentsErr => write!(f, "Not enough arguments provided"),
            CustomError::PathNonExistErr => write!(f, "Path doesn't exist"),
            CustomError::InvalidPassesErr => write!(f, "Passes value is invalid"),
            CustomError::InvalidThreadCountErr => write!(f, "Thread count value is invalid"),
        }
    }
}

impl Error for CustomError {}

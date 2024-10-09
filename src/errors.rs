use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CustomError {
    NotEnoughArgumentsErr,
    PathNonExistErr,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::NotEnoughArgumentsErr => write!(f, "Not enough arguments provided"),
            CustomError::PathNonExistErr => write!(f, "Path doesn't exist"),
        }
    }
}

impl Error for CustomError {}

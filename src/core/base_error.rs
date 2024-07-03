use core::fmt;
use std::error::Error;

pub struct BaseError {
    pub message: String,
}
impl Error for BaseError {}

impl fmt::Debug for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error:  message: {}", self.message)
    }
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error:  message: {}", self.message)
    }
}

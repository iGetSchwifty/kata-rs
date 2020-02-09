use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct KataError {
    pub message: String
}

impl KataError {
    pub fn new(message: &str) -> KataError {
        KataError{message: message.to_string()}
    }
}

impl fmt::Display for KataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for KataError {
    fn description(&self) -> &str {
        &self.message
    }
}
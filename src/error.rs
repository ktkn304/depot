use std::{fmt::{Formatter, Display, Result}, error::Error as StdError};

#[derive(std::fmt::Debug)]
pub struct CustomError {
    message: String,
}
impl CustomError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.message)
    }
}
impl StdError for CustomError {
}

#[derive(std::fmt::Debug)]
pub struct BuiltInCommandError {
    message: String
}

impl BuiltInCommandError {
    pub fn new(message: &str) -> Self {
        return Self {
            message: message.to_owned()
        };
    }
}

impl Display for BuiltInCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.message)
    }
}

impl StdError for BuiltInCommandError {
}

#[derive(std::fmt::Debug)]
pub struct PathStringifyError {
    message: String
}

impl PathStringifyError {
    pub fn new(message: &str) -> Self {
        return Self {
            message: message.to_owned()
        };
    }
}

impl Display for PathStringifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", &self.message)
    }
}

impl StdError for PathStringifyError {
}

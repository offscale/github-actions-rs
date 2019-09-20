use std::error::Error;
use core::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct FileError {
    description : String
}

impl FileError {
    pub fn new(msg: &str) -> FileError {
        FileError { description: msg.to_string() }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for FileError {

    fn description(&self) -> &str {
        "Error reading file"
    }
}

impl From<std::io::Error> for FileError {
    #[inline]
    fn from(error: std::io::Error) -> FileError {
        FileError { description: error.description().to_string() }
    }
}

impl From<serde_yaml::Error> for FileError {
    fn from(error: serde_yaml::Error) -> FileError {
        FileError {
            description : "failed to parse struct".to_string()
        }
    }
}
use std::{fmt::Display, process};

use crate::position::Position;

/// Enum to represent the diferent kinds of errors
#[derive(Debug)]
pub enum ErrorKind {
    IOError,
}

/// Struct to represent an error
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
    pub pos: Option<Position>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:?}: {}",
            match &self.pos {
                None => "".to_string(),
                Some(p) => p.to_string() + "\n",
            },
            self.kind,
            self.msg
        )
    }
}

/// Prints the error nicely to the console and exits with code 1
pub fn handle_error(e: Error) {
    println!("{}", e);
    process::exit(1);
}

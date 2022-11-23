use std::{fmt::Display, io};

use log::error;
use serde::{Deserialize, Serialize};

use crate::{position::Position, token::Token};

/// Enum to represent the diferent kinds of errors
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorKind {
    FileNotFoundError,
    PermissionDeniedError,
    UnexpectedEof { expected: Token },
}

/// Struct to represent an error
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

// TODO: impl From
impl Error {
    /// Create a new [`Error`] from an [`io::ErrorKind`]
    /// at `pos` reading from `filename`
    pub fn from_std_io_errorkind(
        io_errorkind: io::ErrorKind,
        pos: Option<Position>,
        filename: &str,
    ) -> Error {
        match io_errorkind {
            io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => {
                let kind: ErrorKind;
                let msg: &str;
                match io_errorkind {
                    io::ErrorKind::NotFound => {
                        kind = ErrorKind::FileNotFoundError;
                        msg = "File '{}' not found";
                    }
                    io::ErrorKind::PermissionDenied => {
                        kind = ErrorKind::PermissionDeniedError;
                        msg = "Could not open '{}', permission denied"
                    }
                    _ => unreachable!(), // because we match exhaustively in the outer match statement
                }
                error!("{}", msg.replace("{}", filename));
                Error {
                    kind,
                    msg: msg.replace("{}", filename),
                    pos,
                }
            }
            _ => todo!("Return generic error"),
        }
    }
}

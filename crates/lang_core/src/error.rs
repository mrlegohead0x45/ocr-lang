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
        let (kind, msg) = match io_errorkind {
            io::ErrorKind::NotFound => (ErrorKind::FileNotFoundError, "File '{}' not found"),
            io::ErrorKind::PermissionDenied => (
                ErrorKind::PermissionDeniedError,
                "Could not open '{}', permission denied",
            ),

            _ => todo!("Return generic error"),
        };

        let formatted_message = msg.replace("{}", filename);

        error!("{}", formatted_message);

        Error {
            kind,
            msg: formatted_message,
            pos,
        }
    }
}

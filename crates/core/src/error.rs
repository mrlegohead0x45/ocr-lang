use std::{fmt::Display, io};

use log::error;

use crate::position::Position;

/// Enum to represent the diferent kinds of errors
#[derive(Debug)]
pub enum ErrorKind {
    FileNotFoundError,
    PermissionDeniedError,
}

/// Struct to represent an error
pub struct Error<'a> {
    pub kind: ErrorKind,
    pub msg: String,
    pub pos: Option<Position<'a>>,
}

impl Display for Error<'_> {
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
impl Error<'_> {
    /// Create a new [`Error`] from an [`io::ErrorKind`]
    /// at `pos` reading from `filename`
    pub fn from_std_io_errorkind<'a>(
        io_errorkind: io::ErrorKind,
        pos: Option<Position<'a>>,
        filename: &'a str,
    ) -> Error<'a> {
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

use std::io::{self, Read};

use crate::error::{Error, ErrorKind};
use crate::position::Position;
use crate::token::Token;

/// Struct to transform input into [`Token`]s
pub struct Lexer {
    /// Stream that we are reading our input from
    stream: Box<dyn Read>,
    /// Character that we are currently processing
    current_char: Option<char>,
    /// Our position in the text we are lexing
    pos: Position,
    /// The file name that we are lexing
    filename: String,
}

impl Lexer {
    /// Create a new [`Lexer`]
    pub fn new(stream: Box<dyn Read>, filename: String) -> Self {
        Self {
            stream,
            current_char: None,
            pos: Position::start(filename.clone()),
            filename,
        }
    }

    /// Transform into [`Vec<Token>`].
    /// Returns `Err` if we could not parse the input stream.
    /// Access the [`Error`]'s `kind` field for more details
    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while self.current_char.is_some() {
            if self.advance().is_err() {
                return Err(Error {
                    kind: ErrorKind::IOError,
                    msg: format!("Could not open file '{}'", self.filename),
                });
            };
        }

        Ok(tokens)
    }

    /// Advance the [`Lexer`].
    /// Reads one byte from the input stream and places it in
    /// `current_char`. Returns `Err` if reading from the stream failed,
    /// with a `kind` of [`ErrorKind::IOError`]
    fn advance(&mut self) -> io::Result<()> {
        let mut buf = [0; 1];
        match self.stream.read_exact(&mut buf) {
            Ok(()) => {
                self.current_char = Some(buf[0] as char);
                // OK to use unwrap because we set it to Some in the previous line,
                // so something has gone horribly wrong if it's None
                self.pos.advance(self.current_char.unwrap());
                Ok(())
            }
            Err(e) => {
                self.current_char = None;
                Err(e)
            }
        }
    }
}

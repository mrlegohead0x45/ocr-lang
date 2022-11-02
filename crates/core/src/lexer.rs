use std::{io, io::Read};

use log::{trace, warn};

use crate::error::Error;
use crate::position::Position;
use crate::token::Token;
use crate::ErrorKind;

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

/// Type alias for an empty [`Result`]
type EmptyResult = Result<(), Error>;

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
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        loop {
            self.advance()?;

            self.consume_whitespace()?;

            let mut expected: Option<Token> = None;

            if self.current_char.is_none() {
                if let Some(expected) = expected {
                    return Err(Error {
                        kind: ErrorKind::UnexpectedEof {
                            expected: expected.clone(),
                        },
                        msg: format!("Unexpected EOF, expected {:?}", expected),
                        pos: Some(self.pos.clone()),
                    });
                } else {
                    return Ok(tokens);
                }
            }
        }
    }

    /// Keep advancing as long as `self.current_char` is whitespace,
    /// which is any of the following characters
    /// - `' '`
    /// - `'\t'`
    /// - `'\n'`
    /// - `'\r'`
    fn consume_whitespace(&mut self) -> EmptyResult {
        trace!("Consuming whitespace");
        while let Some(' ' | '\t' | '\n' | '\r') = self.current_char {
            self.advance()?;
        }
        Ok(())
    }

    /// Advance the [`Lexer`].
    /// Reads one byte from the input stream and places it in
    /// `current_char`. Returns `Err` if reading from the stream failed,
    /// containing the error encountered.
    fn advance(&mut self) -> EmptyResult {
        let mut buf = [0; 1];
        match self.stream.read_exact(&mut buf) {
            Ok(()) => {
                self.current_char = Some(buf[0] as char);
                // OK to use unwrap because we set it to Some in the previous line,
                // so something has gone horribly wrong if it's None
                self.pos.advance(self.current_char.unwrap());
                Ok(())
            }
            Err(e) => match e.kind() {
                // Eof needs more complex handling
                io::ErrorKind::UnexpectedEof => {
                    warn!("Reached EOF while lexing");
                    self.current_char = None;
                    Ok(())
                }
                _ => Err(Error::from_std_io_errorkind(
                    e.kind(),
                    Some(self.pos.clone()),
                    &self.filename,
                )),
            },
        }
    }
}

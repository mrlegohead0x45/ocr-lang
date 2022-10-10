use std::io::{self, Read};

use crate::error::Error;
use crate::token::Token;

/// Struct to transform input into [`Token`]s
pub struct Lexer {
    /// Stream that we are reading our input from
    stream: Box<dyn Read>,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(stream: Box<dyn Read>) -> Self {
        Self {
            stream: stream,
            current_char: None,
        }
    }

    pub fn lex(&self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while self.current_char.is_some() {}

        Ok(tokens)
    }

    fn advance(&mut self) -> io::Result<()> {
        let mut buf = [0; 1];
        match self.stream.read_exact(&mut buf) {
            Ok(()) => {
                self.current_char = Some(buf[0] as char);
                return Ok(());
            }
            Err(e) => {
                self.current_char = None;
                return Err(e);
            }
        }
    }
}

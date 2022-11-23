use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Struct to represent the position we are at in the text
/// we are lexing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    line: usize,
    column: usize,
    filename: String,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

impl Position {
    /// Return a [`Position`] at the start of a file
    pub fn start(filename: String) -> Self {
        Self {
            line: 1,
            column: 1,
            filename,
        }
    }

    /// Advance this [`Position`] to `char`.
    /// Increments `line` and also `column` if `char` is a newline
    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let pos = Position::start("filename".to_string());
        assert_eq!(pos.column, 1);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.filename, "filename".to_string())
    }

    #[test]
    fn test_advance() {
        let mut pos = Position::start("filename".to_string());
        pos.advance('c');

        assert_eq!(pos.column, 2);
    }

    #[test]
    fn test_advance_lf() {
        let mut pos = Position::start("filename".to_string());
        pos.advance('c');
        pos.advance('\n');

        assert_eq!(pos.column, 0);
        assert_eq!(pos.line, 2);
    }

    #[test]
    fn test_advance_crlf() {
        let mut pos = Position::start("filename".to_string());
        pos.advance('c');
        pos.advance('\r');
        pos.advance('\n');

        assert_eq!(pos.column, 0);
        assert_eq!(pos.line, 2);
    }

    #[test]
    fn test_display() {
        let pos = Position::start("filename".to_string());

        assert_eq!(pos.to_string(), "filename:1:1")
    }
}

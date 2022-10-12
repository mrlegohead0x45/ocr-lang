use std::fmt::Display;

/// Struct to represent the position we are at in the text
/// we are lexing
#[derive(Debug)]
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

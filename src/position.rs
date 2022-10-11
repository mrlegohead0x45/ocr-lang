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
        write!(
            f,
            "Line {}, column {}, '{}'",
            self.line, self.column, self.filename
        )
    }
}

impl Position {
    pub fn start(filename: String) -> Self {
        Self {
            line: 1,
            column: 1,
            filename,
        }
    }

    pub fn advance(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

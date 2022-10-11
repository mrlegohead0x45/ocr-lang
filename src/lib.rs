use std::io;

pub mod error;
pub mod lexer;
mod position;
pub mod token;

pub fn ocr_lang_main() {
    let lexer = lexer::Lexer::new(Box::new(io::stdin()), "<stdin>".to_string());
}

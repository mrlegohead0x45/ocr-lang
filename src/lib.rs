use std::io;

pub mod error;
pub mod lexer;
pub mod token;

pub fn ocr_lang_main() {
    let lexer = lexer::Lexer::new(Box::new(io::stdin()));
}

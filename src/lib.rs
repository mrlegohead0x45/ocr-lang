//! OCR-lang is an interpreter for OCR Exam Reference, the pseudocode found in
//! OCR Computer Science exams, at GCSE and A-Level.
//! At some point in the future, I may try and implement a compiler using LLVM.

use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use log::{trace, LevelFilter};

use crate::error::{handle_error, Error, ErrorKind};

mod args;
mod error;
mod lexer;
mod position;
mod token;

/// The main function for this program.
/// Parses arguments, lexes tokens.
pub fn ocr_lang_main() {
    setup_logging().expect("Init logging failed");

    let args = args::Args::parse();
    let stream: Box<dyn Read> = if args.filename.is_none() {
        Box::new(io::stdin())
    } else {
        Box::new(match File::open(args.filename.as_ref().unwrap()) {
            Err(_) => {
                handle_error(Error {
                    kind: ErrorKind::IOError,
                    msg: format!("Could not open file '{}'", args.filename.unwrap()),
                });
                unreachable!() // because handle_error exits
            }
            Ok(f) => f,
        })
    };

    let lexer = lexer::Lexer::new(
        stream,
        match args.filename {
            None => String::from("<stdin>"),
            Some(f) => f,
        },
    );
    trace!("Constructed lexer");
}

/// Initialise logging.
/// Copy-pasted from [Fern docs](https://docs.rs/fern/latest/fern/)
fn setup_logging() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug)
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

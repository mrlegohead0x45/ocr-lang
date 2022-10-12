//! OCR-lang is an interpreter for OCR Exam Reference, the pseudocode found in
//! OCR Computer Science exams, at GCSE and A-Level.
//! At some point in the future, I may try and implement a compiler using LLVM.

use std::{
    fs::File,
    io::{self, Read},
};

use args::LogLevel;
use clap::Parser;
use log::{error, trace};

use crate::error::{handle_error, Error, ErrorKind};

mod args;
mod error;
mod lexer;
mod position;
mod token;

/// The main function for this program.
/// Parses arguments, lexes tokens.
pub fn ocr_lang_main() {
    let args = args::Args::parse();
    setup_logging(args.log_level).expect("Init logging failed");

    let stream: Box<dyn Read> = if args.filename.is_none() {
        trace!("Reading from stdin");
        Box::new(io::stdin())
    } else {
        trace!("Reading from '{}'", args.filename.as_ref().unwrap());
        Box::new(match File::open(args.filename.as_ref().unwrap()) {
            Err(_) => {
                error!("Could not open file '{}'", args.filename.as_ref().unwrap());
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
fn setup_logging(level: LogLevel) -> Result<(), fern::InitError> {
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
        .level(level.to_level_filter())
        .chain(io::stdout())
        .apply()?;
    Ok(())
}

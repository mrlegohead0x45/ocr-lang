mod args;

use std::{
    fs::File,
    io::{stdin, stdout},
    process,
};

use clap::Parser;
use log::{error, info};

use lang_core::{types::Reader, Error, Lexer};

use crate::args::LogLevel;

fn main() {
    let args = args::Args::parse();
    setup_logging(args.log_level).expect("Init logging failed");

    let stream: Reader = if args.filename.is_none() {
        info!("Reading from stdin");
        Box::new(stdin())
    } else {
        info!("Reading from '{}'", args.filename.as_ref().unwrap());
        Box::new(match File::open(args.filename.as_ref().unwrap()) {
            Err(e) => {
                error!("Could not open file '{}'", args.filename.as_ref().unwrap());
                handle_error(Error::from_std_io_errorkind(
                    e.kind(),
                    None,
                    &args.filename.unwrap(),
                ));
                unreachable!() // because handle_error exits
            }
            Ok(f) => f,
        })
    };

    let mut lexer = Lexer::new(
        stream,
        match args.filename {
            None => String::from("<stdin>"),
            Some(f) => f,
        },
    );
    info!("Constructed lexer");
    let maybe_tokens = lexer.make_tokens();
    match maybe_tokens {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => handle_error(e),
    }
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
        .chain(stdout())
        .apply()?;
    Ok(())
}

/// Print an error and exit code 1
fn handle_error(e: Error) {
    println!("{}", e);
    process::exit(1);
}

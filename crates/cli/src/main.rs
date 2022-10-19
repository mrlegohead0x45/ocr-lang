mod args;

use std::{
    fs::File,
    io::{stdin, stdout, Read},
    process,
};

use clap::Parser;
use log::{error, trace};

use core::{Error, Lexer};

use crate::args::LogLevel;

fn main() {
    let args = args::Args::parse();
    setup_logging(args.log_level).expect("Init logging failed");

    let stream: Box<dyn Read> = if args.filename.is_none() {
        trace!("Reading from stdin");
        Box::new(stdin())
    } else {
        trace!("Reading from '{}'", args.filename.as_ref().unwrap());
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

    let lexer = Lexer::new(
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
        .chain(stdout())
        .apply()?;
    Ok(())
}

/// Print an error and exit code 1
fn handle_error(e: Error) {
    println!("{}", e);
    process::exit(1);
}

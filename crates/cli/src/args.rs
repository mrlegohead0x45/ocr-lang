use clap::Parser;

#[derive(Parser)]
#[command(name = "ocr-lang")]
#[command(author = "mrlegohead0x45 <mrlegohead0x45@gmail.com>")]
#[command(version = clap::crate_version!())]
#[command(about = clap::crate_description!())]
pub struct Args {
    /// File to run.
    /// If not given, defaults to stdin, which does not mean a REPL,
    /// rather the code will be read from stdin all at once and then run.
    pub filename: Option<String>,

    /// Level to log messages at
    #[arg(value_enum, short, long, default_value_t = LogLevel::Off)]
    pub log_level: LogLevel,

    /// Specification to run the code at
    #[arg(value_enum, short, long, default_value_t = Specification::A2)]
    pub spec: Specification,
}

/// Enum for logging levels in args.
/// This is needed, because we could `derive` [`clap::ValueEnum`] for
/// [`log::LevelFilter`], but somebody already tried to submit a
/// [PR](https://github.com/rust-lang/log/pull/524) for that and got rejected
#[derive(Clone, clap::ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    /// Convert one-to-one to [`log::LevelFilter`]
    pub fn to_level_filter(&self) -> log::LevelFilter {
        match self {
            Self::Off => log::LevelFilter::Off,
            Self::Error => log::LevelFilter::Error,
            Self::Warn => log::LevelFilter::Warn,
            Self::Info => log::LevelFilter::Info,
            Self::Debug => log::LevelFilter::Debug,
            Self::Trace => log::LevelFilter::Trace,
        }
    }
}

/// Enum for what specification to run the code at.
#[derive(clap::ValueEnum, Clone)]
pub enum Specification {
    /// Core language.
    Gcse,
    /// AS-Level. GCSE with pass-by-value and -reference
    As,
    /// A-Level. AS with OOP
    A2,
}

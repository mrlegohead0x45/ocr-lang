use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// File to run.
    /// If not given, defaults to stdin, which does not mean a REPL,
    /// rather the code will be read from stdin all at once and then run.
    pub filename: Option<String>,

    /// Level to log messages at
    #[arg(value_enum, short, long, default_value_t = LogLevel::Off)]
    pub log_level: LogLevel,
}

/// This is needed.
/// See rust-lang/log#524
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

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// File to run.
    /// If not given, defaults to stdin, which does not mean a REPL,
    /// rather the code will be read from stdin all at once and then run.
    pub filename: Option<String>,
}

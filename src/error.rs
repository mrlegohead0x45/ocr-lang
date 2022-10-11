use std::process;

/// Enum to represent the diferent kinds of errors
#[derive(Debug)]
pub enum ErrorKind {
    IOError,
}

/// Struct to represent an error
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
}

/// Prints the error nicely to the console and exits with code 1
pub fn handle_error(e: Error) {
    println!("{:?}: {}", e.kind, e.msg);
    process::exit(1);
}

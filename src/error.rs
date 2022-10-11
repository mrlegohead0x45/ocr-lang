/// Enum to represent the diferent kinds of errors
pub enum ErrorKind {
    IOError,
}

/// Struct to represent an error
pub struct Error {
    pub kind: ErrorKind,
}

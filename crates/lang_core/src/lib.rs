#![allow(rustdoc::private_intra_doc_links)]

mod error;
mod lexer;
mod position;
mod token;

pub use error::{Error, ErrorKind};
pub use lexer::Lexer;
pub use token::Token;

pub mod types {
    //! Module for type aliases

    /// Type alias for a [`Box`]ed [`std::io::Read`]
    pub type Reader = Box<dyn std::io::Read>;

    /// Type alias of [`Result`] for our [`crate::Error`] type
    pub type Result<T> = ::core::result::Result<T, crate::Error>;

    /// Type alias for an empty [`Result`]
    pub type EmptyResult = Result<()>;
}

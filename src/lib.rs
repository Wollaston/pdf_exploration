//!
//! An experimental library project exploring representing portions of the PDF standard
//! using Rust.
//!
//! The general idea:
//!     - A PDF is a binary format, so read it in as bytes
//!     - Use a crate like bstr to read through the bytes
//!         and determine the byte offset of keyword delimiters
//!         of PDF objects (this is done using a crate like bstr
//!         as not all bytes are valid UTF-8)
//!     - Once the byte offset and type of object match is determined,
//!         use custom nom functions to parse the data into defined
//!         structs representing the key data in the PDF objects
//!
//! Once that is done, experiment with modifying data, etc., and implementing
//! new fetures for other parts of the PDF standard.
//!
//! Work in progress. For learning purposes.
//!
//! The PDF standard is available online as ISO 32000-2:2020
//!
pub mod api;

//! # Chrono-utils
//! Utils for the [chrono](https://github.com/lifthrasiir/rust-chrono) library.
//! 
//! ## Supported features
//!
//! * W3C parser and formatter
//#
extern crate chrono;
/// Using for date and time parsing.
pub mod parser;
/// Using for date and time formatting.
pub mod formatter;
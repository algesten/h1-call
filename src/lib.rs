//! Supporting crate for [ureq](https://crates.io/crates/ureq).
//!
//! This crate contains types used to implement ureq.
//!
//!

#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::needless_lifetimes)]
#![deny(missing_docs)]

extern crate alloc;

#[macro_use]
extern crate log;

// Re-export the basis for this library.
pub use http;

mod error;
pub use error::Error;

mod chunk;
mod ext;
mod util;

mod body;
pub use body::BodyMode;

pub mod client;

/// Low level HTTP parser
///
/// This is to bridge `httparse` crate to `http` crate.
pub mod parser;

#[doc(hidden)]
pub use util::ArrayVec;

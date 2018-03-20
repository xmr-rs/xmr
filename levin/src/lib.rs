#![deny(anonymous_parameters, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unreachable_pub, unsafe_code, unstable_features, unused_extern_crates, unused_import_braces, unused_qualifications)]

//! # xmr-levin
//!
//! Rust implementation of the levin protocol used in the Monero cryptocurrency.
//!
//! # Usage:
//!
//! Add it to the dependencies in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! xmr-levin = { git = "https://www.github.com/xmr-rs/xmr", version = "0.1.0" }
//! ```
//!
//! And import it in your crate like this:
//!
//! ```rust
//! extern crate xmr_levin;
//! // or if you're using it inside a crate of the xmr project:
//! extern crate xmr_levin as levin;
//! ```
//!

#[macro_use]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate crossbeam;
extern crate parking_lot;

extern crate bytes;

#[macro_use]
extern crate failure_derive;
extern crate failure;

#[macro_use]
extern crate log;

extern crate xmr_portable_storage as portable_storage;

pub mod bucket;
pub mod net;

mod command;
mod error;

pub use command::{COMMAND_BASE_ID, Command, Id};
pub use error::{BucketHeadError, Error, Result};

#![deny(anonymous_parameters, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unreachable_pub, unsafe_code, unstable_features, unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

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
extern crate tokio_io;

extern crate bytes;

#[macro_use]
extern crate failure_derive;
extern crate failure;

extern crate serde;

#[macro_use]
extern crate log;

extern crate xmr_portable_storage as portable_storage;

pub mod bucket;

mod command;
mod error;
mod storage;

pub use command::{COMMAND_BASE_ID, Command, Notify};
pub use error::{BucketHeadError, Error, Result};
pub use storage::{Storage, Empty};

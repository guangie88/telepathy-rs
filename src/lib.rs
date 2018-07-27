#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations)]

extern crate emojicons;
#[macro_use]
extern crate derive_more;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

pub mod error;
pub mod file;
pub mod stickers;
pub mod util;

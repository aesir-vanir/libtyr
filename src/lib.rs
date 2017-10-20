// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libtyr` ORM and Query Builder generation.
#![deny(missing_docs)]
#![feature(inclusive_range_syntax)]
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_derive;

extern crate inflector;
extern crate mimir;
extern crate mustache;
extern crate rustc_serialize;
extern crate term;

mod code;
mod dam;
#[allow(missing_docs)]
mod error;
mod util;

pub use dam::{fetch, ColumnMetadata, Context, ContextBuilder, RowMetadata, RowsMetadata, TablesMetadata};
pub use error::{Error, ErrorKind};
pub use code::{Gen, GenBuilder};
pub use util::pretty_print;

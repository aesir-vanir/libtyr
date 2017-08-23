//! `libtyr` ORM and Query Builder generation.
#![deny(missing_docs)]
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;

extern crate mimir;
extern crate term;

mod dam;
#[allow(missing_docs)]
mod error;
mod util;

pub use dam::{fetch, ColumnMetadata, ContextBuilder, RowMetadata, RowsMetadata, TablesMetadata};
pub use error::{Error, ErrorKind};
pub use util::pretty_print;

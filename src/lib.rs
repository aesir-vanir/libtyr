//! `libtyr` ORM and Query Builder generation.
#![deny(missing_docs)]
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;

extern crate mimir;

mod dam;
mod error;

pub use dam::{fetch, ColumnMetadata, ContextBuilder, RowMetadata, RowsMetadata, TablesMetadata};

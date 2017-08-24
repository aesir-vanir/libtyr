//! `libtyr` ORM and Query Builder generation.
#![deny(missing_docs)]
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
extern crate serde;
extern crate term;

mod dam;
#[allow(missing_docs)]
mod error;
mod render;
mod util;

pub use dam::{fetch, ColumnMetadata, Context, ContextBuilder, RowMetadata, RowsMetadata,
              TablesMetadata};
pub use error::{Error, ErrorKind};
pub use render::{Render, RenderBuilder};
pub use util::pretty_print;

//! Render tables metadata as Rust ORM code via mustache.
use dam::TablesMetadata;
use error::Result;
use std::io::Write;

/// Render Configuration
#[derive(Builder, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Render {
    /// Should a file be created per table?
    file_per_table: bool,
}

impl Render {
    /// Render the given tables metadata as rust code to the given writer.
    pub fn render<W>(&self, _table_metadata: &TablesMetadata, _writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        Ok(())
    }
}

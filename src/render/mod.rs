//! Render tables metadata as Rust ORM code via mustache.
use dam::TablesMetadata;
use error::Result;
use inflector::cases::snakecase::to_snake_case;
use inflector::cases::pascalcase::to_pascal_case;
use mustache;
use std::io::Write;

mod tmpl;

use self::tmpl::{Derive, DeriveBuilder, Field, File, Table, TableBuilder, ORM_TMPL};

/// Render Configuration
#[derive(Builder, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Render {
    /// Should a file be created per table?
    file_per_table: bool,
}

impl Render {
    /// Render the given tables metadata as rust code to the given writer.
    pub fn render<W>(&self, table_metadata: &TablesMetadata, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        let template = mustache::compile_str(ORM_TMPL)?;
        let mut file: File = Default::default();
        let mut tables: Vec<Table> = Vec::new();

        for (table_name, rows) in table_metadata {
            let mut fields: Vec<Field> = Vec::new();

            for col_info in rows.values() {
                let mut field: Field = Default::default();

                for col in col_info {
                    match &(*col.column_name())[..] {
                        "COLUMN_NAME" => {
                            let type_info = col.type_info();
                            let data = if let Some(ref data) = *col.data() {
                                data.to_string(type_info)?
                            } else {
                                "(null)".to_string()
                            };
                            field.set_field_name(to_snake_case(&data));
                        }
                        "DATA_TYPE" => {
                            let type_info = col.type_info();
                            let data = if let Some(ref data) = *col.data() {
                                data.to_string(type_info)?
                            } else {
                                "(null)".to_string()
                            };
                            let mapped = match &data[..] {
                                "NUMBER" => "f64",
                                "VARCHAR2" => "String",
                                _ => "",
                            };
                            field.set_field_type(mapped.to_string());
                        }
                        "NULLABLE" => {
                            let type_info = col.type_info();
                            let data = if let Some(ref data) = *col.data() {
                                data.to_string(type_info)?
                            } else {
                                "(null)".to_string()
                            };
                            match &data[..] {
                                "Y" => {
                                    let mut optional = String::from("Option<");
                                    optional.push_str(field.field_type());
                                    optional.push_str(">");
                                    field.set_field_type(optional);
                                    field.set_nullable(true)
                                }
                                "N" => field.set_nullable(false),
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                fields.push(field);
            }
            let mut derives = Vec::new();
            let derive_names = vec!["Clone", "Default", "Debug", "Eq", "Hash", "PartialEq"];
            for (idx, derive) in derive_names.iter().enumerate() {
                let derive: Derive = DeriveBuilder::default()
                    .name(derive.to_string())
                    .comma(idx < (derive_names.len() - 1))
                    .build()?;
                derives.push(derive);
            }

            let table: Table = TableBuilder::default()
                .struct_name(to_pascal_case(table_name))
                .derives(derives)
                .field(fields)
                .build()?;

            tables.push(table);
        }
        file.set_tables(tables);
        template.render(writer, &file)?;
        Ok(())
    }
}

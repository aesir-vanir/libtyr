// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Render tables metadata as Rust ORM code via mustache.
use dam::{ColumnMetadata, RowsMetadata, TablesMetadata};
use error::{ErrorKind, Result};
use inflector::cases::snakecase::to_snake_case;
use inflector::cases::pascalcase::to_pascal_case;
use mustache;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;

mod tmpl;

use self::tmpl::{Derive, DeriveBuilder, Field, ModBuilder, Table, TableBuilder, TableModBuilder, MOD_RS, TABLE_RS};

/// Render Configuration
#[derive(Builder, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Gen {
    /// The relative module path for the generated files.
    module_path: PathBuf,
    /// Should the generated code include query building?
    query_builder: bool,
}

impl Gen {
    /// Generate the module code for the given `TablesMetadata`.
    pub fn gen(&self, tables_metadata: &TablesMetadata) -> Result<()> {
        fs::create_dir_all(&self.module_path)?;
        mod_gen(&self.module_path, tables_metadata)?;

        for (table_name, rows) in tables_metadata {
            table_gen(&self.module_path, table_name, rows)?
        }
        Ok(())
    }
}

/// Generate Rust code for main level module.
pub fn mod_gen(path: &PathBuf, tables_metadata: &TablesMetadata) -> Result<()> {
    let template = mustache::compile_str(MOD_RS)?;
    let mut table_mods = Vec::new();

    for table_name in tables_metadata.keys() {
        let lc_table_name: String = table_name
            .chars()
            .map(|c| c.to_lowercase().to_string())
            .collect();
        table_mods.push(TableModBuilder::default().name(lc_table_name).build()?);
    }

    let dam_mod = ModBuilder::default()
        .name("dam".to_string())
        .tables(table_mods)
        .build()?;
    let mut module_path = PathBuf::from(path);
    module_path.push("mod.rs");

    let mod_file = File::create(module_path)?;
    let mut writer = BufWriter::new(mod_file);
    template.render(&mut writer, &dam_mod)?;

    Ok(())
}

/// Generate Rust code for the given `RowsMetadata`.
pub fn table_gen(path: &PathBuf, table_name: &str, rows: &RowsMetadata) -> Result<()> {
    let template = mustache::compile_str(TABLE_RS)?;
    let mut fields: Vec<Field> = Vec::new();

    for col_info in rows.values() {
        let mut field: Field = Default::default();

        for col in col_info {
            match &(*col.column_name())[..] {
                "COLUMN_NAME" => field.set_field_name(to_snake_case(&data_as_str(col)?)),
                "DATA_TYPE" => field.set_field_type(map_data_type(&data_as_str(col)?)?),
                "NULLABLE" => map_nullable(&data_as_str(col)?, &mut field)?,
                _ => {}
            }
        }
        fields.push(field);
    }
    let mut derives = Vec::new();
    let derive_names = vec!["Clone", "Default", "Debug", "PartialEq"];
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

    let lc_table_name: String = table_name
        .chars()
        .map(|c| c.to_lowercase().to_string())
        .collect();
    let mut module_path = PathBuf::from(path);
    module_path.push(lc_table_name);
    module_path.set_extension("rs");

    let table_file = File::create(module_path)?;
    let mut writer = BufWriter::new(table_file);
    template.render(&mut writer, &table)?;
    Ok(())
}

/// Get the data from `ColumnMetadata` as a `String`.
fn data_as_str(col: &ColumnMetadata) -> Result<String> {
    let data = if let Some(ref data) = *col.data() {
        data.get_string()
    } else {
        "(null)".to_string()
    };
    Ok(data)
}

/// Map the Oracle data type to a Rust data type.
fn map_data_type(data_type: &str) -> Result<String> {
    let mapped = match data_type {
        "NUMBER" => "f64",
        "VARCHAR2" => "String",
        _ => "",
    };
    Ok(mapped.to_string())
}

/// Map a nullable column to an `Option` field.
fn map_nullable(data_str: &str, field: &mut Field) -> Result<()> {
    match data_str {
        "Y" => {
            let mut optional = String::from("Option<");
            optional.push_str(field.field_type());
            optional.push_str(">");
            field.set_field_type(optional);
            field.set_nullable(true);
        }
        "N" => field.set_nullable(false),
        _ => return Err(ErrorKind::Nullable.into()),
    }

    Ok(())
}

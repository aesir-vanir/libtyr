// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libtyr` data access module
use error::Result;
use mimir::enums::ODPINativeTypeNum::Bytes;
use mimir::enums::ODPIOracleTypeNum::Varchar;
use mimir::{self, flags, Connection, Data, TypeInfo};
use std::collections::BTreeMap;

/// `tyr` Oracle database context
#[derive(Builder, Debug, Getters)]
pub struct Context {
    /// `mimir` context
    #[get]
    #[builder(default = "self.default_db_context()?")]
    db_context: mimir::Context,
    /// Connection string.
    #[get]
    conn_string: String,
    /// Username use for db connection.
    #[get]
    username: String,
    /// Password used for db connection.
    #[get]
    password: String,
}

impl ContextBuilder {
    /// Generate the default db context.
    fn default_db_context(&self) -> ::std::result::Result<mimir::Context, String> {
        Ok(mimir::ContextBuilder::default().build()?)
    }
}

/// User space table names query.
const TABLE_NAMES: &'static str = r"select table_name from user_tables";

/// Describe user space tables Oracle SQL.
const DESC: &'static str = r"SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE, DATA_TYPE_MOD,
DATA_TYPE_OWNER, DATA_LENGTH, DATA_PRECISION, DATA_SCALE, NULLABLE, COLUMN_ID, DEFAULT_LENGTH,
NUM_DISTINCT, LOW_VALUE, HIGH_VALUE, DENSITY, NUM_NULLS, NUM_BUCKETS, LAST_ANALYZED, SAMPLE_SIZE,
CHARACTER_SET_NAME, CHAR_COL_DECL_LENGTH, GLOBAL_STATS, USER_STATS, AVG_COL_LEN, CHAR_LENGTH,
CHAR_USED, V80_FMT_IMAGE, DATA_UPGRADED, HISTOGRAM, DEFAULT_ON_NULL, IDENTITY_COLUMN,
EVALUATION_EDITION, UNUSABLE_BEFORE, UNUSABLE_BEGINNING
FROM user_tab_columns
WHERE table_name=:table_name";

/// Column Metadata
#[derive(Debug, Default, Getters, Setters)]
pub struct ColumnMetadata {
    /// The column name.
    #[get = "pub"]
    #[set]
    column_name: String,
    /// The column type information.
    #[get = "pub"]
    #[set]
    type_info: TypeInfo,
    /// The column data.
    #[get = "pub"]
    #[set]
    data: Option<Data>,
}

/// A sorted map of table name to `RowsMetadata`.
pub type TablesMetadata = BTreeMap<String, RowsMetadata>;

/// A sorted map of row index to `RowMetadata`.
pub type RowsMetadata = BTreeMap<u32, RowMetadata>;

/// A row is a vector of `ColumnMetadata`.
pub type RowMetadata = Vec<ColumnMetadata>;

/// Fetch a map of tables column metadata from the database described by the given `Context`.
pub fn fetch(ctxt: &Context) -> Result<TablesMetadata> {
    let mut table_map: TablesMetadata = BTreeMap::new();
    let db_ctxt = ctxt.db_context();
    let mut common_create_params = db_ctxt.init_common_create_params()?;
    common_create_params.set_encoding("UTF-8")?;
    common_create_params.set_nchar_encoding("UTF-8")?;
    common_create_params.set_create_mode(flags::DPI_MODE_CREATE_EVENTS);
    let conn = Connection::create(
        db_ctxt,
        Some(ctxt.username()),
        Some(ctxt.password()),
        Some(ctxt.conn_string()),
        Some(common_create_params),
        None,
    )?;

    let user_tables = conn.prepare_stmt(Some(TABLE_NAMES), None, false)?;
    let _ = user_tables.execute(flags::DPI_MODE_EXEC_DEFAULT)?;
    let (mut found, _) = user_tables.fetch()?;

    while found {
        let (_id_type, data) = user_tables.get_query_value(1)?;
        table_map.insert(data.get_string(), Default::default());
        let (f, _) = user_tables.fetch()?;
        found = f;
    }

    for (table, rows) in &mut table_map {
        let table_desc = conn.prepare_stmt(Some(DESC), None, false)?;
        let table_name_var = conn.new_var(Varchar, Bytes, 1, 256, false, false)?;
        table_name_var.set_from_bytes(0, table)?;
        table_desc.bind_by_name(":table_name", &table_name_var)?;

        let cols = table_desc.execute(flags::DPI_MODE_EXEC_DEFAULT)?;
        let (mut found, mut buffer_row_index) = table_desc.fetch()?;

        while found {
            let mut row_data = Vec::new();
            for i in 1..(cols + 1) {
                let mut query_data_by_col: ColumnMetadata = Default::default();
                let query_info = table_desc.get_query_info(i)?;
                let (_, data) = table_desc.get_query_value(i)?;
                query_data_by_col.set_column_name(query_info.name());
                query_data_by_col.set_type_info(query_info.type_info());
                if !data.null() {
                    query_data_by_col.set_data(Some(data));
                }
                row_data.push(query_data_by_col);
            }

            rows.insert(buffer_row_index, row_data);
            let (f, b) = table_desc.fetch()?;
            found = f;
            buffer_row_index = b;
        }
    }

    user_tables.close(None)?;

    Ok(table_map)
}

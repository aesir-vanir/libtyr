// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Mustache Template for ORM generation.

/// `File` information used for mustache template.
#[derive(Builder, Clone, Debug, Default, Deserialize, RustcEncodable, Eq, Hash, PartialEq,
         Serialize, Setters)]
pub struct File {
    /// Tables to include in this template.
    #[set = "pub"]
    tables: Vec<Table>,
}

/// `Derive` information used for mustache template.
#[derive(Builder, Clone, Debug, Default, Deserialize, RustcEncodable, Eq, Getters, Hash,
         PartialEq, Serialize)]
pub struct Derive {
    /// The derive name.
    name: String,
    /// Include a trailing comma?
    comma: bool,
}

/// `Table` information used for mustache template.
#[derive(Builder, Clone, Debug, Default, Deserialize, RustcEncodable, Eq, Getters, Hash,
         PartialEq, Serialize)]
pub struct Table {
    /// The list of derives.
    derives: Vec<Derive>,
    /// The struct name tag.
    struct_name: String,
    /// The struct fields.
    field: Vec<Field>,
}

/// `Field` information used for mustache template.
#[derive(Clone, Debug, Default, Deserialize, Eq, Getters, Hash, PartialEq, RustcEncodable,
         Serialize, Setters)]
pub struct Field {
    /// The field name tag.
    #[set = "pub"]
    field_name: String,
    /// The field type tag.
    #[set = "pub"]
    #[get = "pub"]
    field_type: String,
    /// Is this field nullable?
    #[set = "pub"]
    nullable: bool,
}

/// Table struct mustache template.
pub const ORM_TMPL: &'static str = "//! ORM generated by tyr
use error::Result;
use mimir::Connection;

pub mod context;

{{#tables}}/// `{{struct_name}}` ORM
#[derive(Getters, MutGetters, Setters, {{#derives}}{{name}}{{#comma}}, {{/comma}}{{/derives}})]
pub struct {{struct_name}} {
    {{#field}}
    /// `{{field_name}}` column
    #[get = \"pub\"]
    #[set = \"pub\"]
    #[get_mut = \"pub\"]
    {{field_name}}: {{{field_type}}},
    {{/field}}
}

impl {{struct_name}} {
    /// Fetch a vector of `{{struct_name}}` from the given connection.  By default, all rows will be
    /// fetched.
    pub fn fetch(_conn: &Connection) -> Result<Vec<{{struct_name}}>> {
        Ok(Vec::new())
    }
}
{{/tables}}";

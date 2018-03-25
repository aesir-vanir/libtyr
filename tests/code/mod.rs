// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use libtyr::{self, Context, GenBuilder};
use std::{env, fs};

#[test]
fn render() {
    let context: Context = ::setup_context().expect("Unable to setup context!");
    let tables_metadata = libtyr::fetch(&context).expect("Unable to fetch metadata!");
    let mut tmpdir = env::temp_dir();
    tmpdir.push("dam");
    if let Ok(codegen) = GenBuilder::default().module_path(tmpdir).query_builder(false).build() {
        assert!(codegen.gen(&tables_metadata).is_ok());

        for table_name in tables_metadata.keys() {
            let lc_table_name: String = table_name.chars().map(|c| c.to_lowercase().to_string()).collect();
            let mut table_code = env::temp_dir();
            table_code.push("dam");
            table_code.push(lc_table_name);
            table_code.set_extension("rs");
            assert!(fs::metadata(table_code).is_ok(), "Error generating table code");
        }
    } else {
        assert!(false, "RenderBuilder build failed!");
    }
}

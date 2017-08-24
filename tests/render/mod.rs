// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use libtyr::{self, Context, RenderBuilder};
use std::io::Cursor;

#[test]
fn render() {
    let context: Context = ::setup_context().expect("Unable to setup context!");
    let tables_metadata = libtyr::fetch(&context).expect("Unable to fetch metadata!");
    let mut output = Cursor::new(Vec::new());
    if let Ok(renderer) = RenderBuilder::default().file_per_table(false).build() {
        match renderer.render(&tables_metadata, &mut output) {
            Ok(_) => {
                let vec = output.into_inner();
                let out_str = String::from_utf8_lossy(&vec);
                assert!(out_str.starts_with("//! ORM generated by tyr"));
            }
            Err(e) => assert!(false, e),
        }
    } else {
        assert!(false, "RenderBuilder build failed!");
    }
}

use libtyr::{self, Context, RenderBuilder};
use std::io::Cursor;

#[test]
fn render() {
    let context: Context = ::setup_context().expect("Unable to setup context!");
    let tables_metadata = libtyr::fetch(&context).expect("Unable to fetch metadata!");
    let mut output = Cursor::new(Vec::new());
    if let Ok(renderer) = RenderBuilder::default().file_per_table(false).build() {
        match renderer.render(&tables_metadata, &mut output) {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, e),
        }
    } else {
        assert!(false, "RenderBuilder build failed!");
    }
}

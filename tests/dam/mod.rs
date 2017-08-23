use libtyr::{self, Context};

#[test]
fn context() {
    match ::setup_context() {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn fetch() {
    let context: Context = ::setup_context().expect("Unable to setup context!");

    match libtyr::fetch(&context) {
        Ok(tables_metadata) => {
            assert_eq!(tables_metadata.len(), 1);

            for (table_name, rows_metadata) in tables_metadata {
                assert_eq!(table_name, "USERNAME");
                assert_eq!(rows_metadata.len(), 2);

                for (idx, (row_idx, row_metadata)) in rows_metadata.iter().enumerate() {
                    assert_eq!(idx, (*row_idx) as usize);
                    assert_eq!(row_metadata.len(), 34);

                    for (col_idx, row) in row_metadata.iter().enumerate() {
                        match col_idx {
                            1 => assert_eq!(row.column_name(), "COLUMN_NAME"),
                            2 => assert_eq!(row.column_name(), "DATA_TYPE"),
                            5 => assert_eq!(row.column_name(), "DATA_LENGTH"),
                            8 => assert_eq!(row.column_name(), "NULLABLE"),
                            _ => {}
                        }
                    }
                }
            }
        }
        Err(_) => assert!(false),
    }
}

#[test]
fn pretty_print() {
    let context: Context = ::setup_context().expect("Unable to setup context!");
    let tables_metadata = libtyr::fetch(&context).expect("Unable to fetch metadata!");
    match libtyr::pretty_print(&tables_metadata) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

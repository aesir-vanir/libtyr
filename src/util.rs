// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libtyr` utilities
use error::{ErrorKind, Result};
use dam::TablesMetadata;
use term;

/// Pad a string to the given length.
fn pad_left(len: usize, s: &str) -> String {
    let mut output = String::new();
    let s_len = s.len();
    if s_len < len {
        let pad_len = len - s_len;
        for _ in 0..pad_len {
            output.push(' ');
        }
    }
    output.push_str(s);

    output
}

/// Pretty print table metadata.
pub fn pretty_print(tables: &TablesMetadata) -> Result<()> {
    for (table, rows) in tables {
        let mut t = term::stdout().ok_or_else(|| ErrorKind::Stdout)?;
        t.attr(term::Attr::Bold)?;
        t.fg(term::color::GREEN)?;
        let table_name = format!(" Table '{}' ", table);
        writeln!(t, "{:#^80}", table_name)?;
        t.reset()?;
        t.flush()?;

        for (idx, col_data) in rows {
            t.fg(term::color::YELLOW)?;
            t.attr(term::Attr::Bold)?;
            let mut row_name = String::from(" Row ");
            row_name.push_str(&(idx + 1).to_string());
            row_name.push(' ');
            writeln!(t, "{:-^80}", row_name)?;
            t.reset()?;
            t.flush()?;

            let max_col_length = col_data
                .iter()
                .map(|col| col.column_name().len())
                .max()
                .ok_or_else(|| ErrorKind::Max)?;

            for col in col_data {
                t.fg(term::color::GREEN)?;
                t.attr(term::Attr::Bold)?;
                let padded_col_name = pad_left(max_col_length, col.column_name());
                write!(t, "{}: ", padded_col_name)?;
                t.reset()?;
                t.flush()?;
                t.fg(term::color::GREEN)?;
                let data = if let Some(ref data) = *col.data() {
                    data.get_string()
                } else {
                    "(null)".to_string()
                };
                writeln!(t, "{}", data)?;
                t.reset()?;
                t.flush()?;
            }

            if (*idx as usize) < rows.len() - 1 {
                writeln!(t, "")?;
            }
        }
    }

    Ok(())
}

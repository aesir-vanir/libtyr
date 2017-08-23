#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

extern crate libtyr;

mod dam;
mod error;

use error::Result;
use libtyr::{Context, ContextBuilder};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CONN_STRING: &'static str = "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL";

#[cfg(test)]
lazy_static! {
    pub static ref CREDS: Vec<String> = {
        let mut creds = Vec::new();
        if let Ok(file) = File::open(".creds/oic-test") {
            let buf_reader = BufReader::new(file);

            #[cfg_attr(feature = "cargo-clippy", allow(used_underscore_binding))]
            for line_res in buf_reader.lines() {
                if let Ok(line) = line_res {
                    let parts = line.split(':').map(|x| {
                        x.trim_right().to_string()
                    }).collect::<Vec<String>>();
                    creds.extend(parts);
                }
            }
        } else {
            let username = env::var("MIMIR_USERNAME").expect("invalid username");
            let password = env::var("MIMIR_PASSWORD").expect("invalid password");
            creds.push(username);
            creds.push(password);

            let odpic_username = env::var("ODPIC_USERNAME").expect("invalid username");
            let odpic_password = env::var("ODPIC_PASSWORD").expect("invalid password");
            creds.push(odpic_username);
            creds.push(odpic_password);
        }
        creds
    };
}

#[cfg(test)]
fn setup_context() -> Result<Context> {
    Ok(ContextBuilder::default()
        .conn_string(CONN_STRING.to_string())
        .username(CREDS[0].to_string())
        .password(CREDS[1].to_string())
        .build()?)
}
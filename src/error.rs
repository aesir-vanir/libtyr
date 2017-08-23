//! `libtyr` errors

error_chain! {
    foreign_links {
        Mimir(::mimir::error::Error);
    }
}

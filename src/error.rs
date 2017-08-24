//! `libtyr` errors

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Mimir(::mimir::error::Error);
        Mustache(::mustache::Error);
        Term(::term::Error);
    }

    errors {
        Max {
            description("")
            display("")
        }
        Nullable {
            description("")
            display("")
        }
        Stdout {
            description("")
            display("")
        }
    }
}

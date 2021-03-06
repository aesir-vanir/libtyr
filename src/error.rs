// Copyright (c) 2017 libtyr developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `libtyr` errors

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Mimir(::mimir::Error);
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

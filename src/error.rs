// Copyright (c) 2017 repomon-config developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `repomon-config` errors
error_chain!{
    foreign_links {
        Io(::std::io::Error);
        TomlDe(::toml::de::Error);
        TomlSer(::toml::ser::Error);
    }
}
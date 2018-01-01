// Copyright (c) 2017 repomon-config developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Configuration management for repomon.
//!
//! # Examples
//!
//! ## Read from TOML string
//!
//! ```
//! # #[macro_use] extern crate error_chain;
//! # extern crate repomon;
//! # extern crate toml;
//! #
//! # use repomon::{read_toml, Branches};
//! # use std::io::Cursor;
//! #
//! # mod error {
//! #     error_chain!{
//! #         foreign_links {
//! #             Io(::std::io::Error);
//! #             RepomonConfig(::repomon::Error);
//! #             TomlDe(::toml::de::Error);
//! #             TomlSer(::toml::ser::Error);
//! #         }
//! #     }
//! # }
//! #
//! # fn main() {}
//! #
//! # fn read() -> error::Result<()> {
//!     let test_toml = r#"[[branch.blah]]
//!     name = "origin/master"
//!     interval = "1m"
//!
//!     [[branch.repomon]]
//!     name = "origin/master"
//!     interval = "1m"
//!
//!     [[branch.repomon]]
//!     name = "origin/feature/testing"
//!     interval = "1m"
//!
//!     [[branch.repomon-config]]
//!     name = "origin/master"
//!     interval = "1m"
//!     "#;
//!
//!     // Serialize the TOML above into a `Branches` struct.
//!     let mut reader = Cursor::new(test_toml);
//!     let branches = read_toml(&mut reader)?;
//!
//!     // Check the `Branches` struct.
//!     let branch_map = branches.branch_map();
//!     assert_eq!(branch_map.keys().len(), 3);
//!     assert!(branch_map.contains_key("repomon"));
//!     assert!(branch_map.contains_key("repomon-config"));
//!     assert!(branch_map.contains_key("blah"));
//!
//!     // Check we have the right number of branch definitions per repo.
//!     let mut branches = branch_map.get("repomon").ok_or("invalid key")?;
//!     assert_eq!(branches.len(), 2);
//!     branches = branch_map.get("repomon-config").ok_or("invalid key")?;
//!     assert_eq!(branches.len(), 1);
//!     branches = branch_map.get("blah").ok_or("invalid key")?;
//!     assert_eq!(branches.len(), 1);
//! #   Ok(())
//! # }
//! ```
//!
//! ## Write to TOML string
//!
//! ```
//! # #[macro_use] extern crate error_chain;
//! # extern crate repomon;
//! # extern crate toml;
//! #
//! # use repomon::{write_toml, Branch, Branches};
//! # use std::collections::BTreeMap;
//! # use std::io::Cursor;
//! #
//! # mod error {
//! #     error_chain!{
//! #         foreign_links {
//! #             Io(::std::io::Error);
//! #             RepomonConfig(::repomon::Error);
//! #             TomlDe(::toml::de::Error);
//! #             TomlSer(::toml::ser::Error);
//! #         }
//! #     }
//! # }
//! #
//! # const TEST_TOML: &str = r#"[[branch.blah]]
//! # name = "origin/master"
//! # interval = "1m"
//! #
//! # [[branch.repomon]]
//! # name = "origin/master"
//! # interval = "1m"
//! #
//! # [[branch.repomon]]
//! # name = "origin/feature/testing"
//! # interval = "1m"
//! #
//! # [[branch.repomon-config]]
//! # name = "origin/master"
//! # interval = "1m"
//! # "#;
//! #
//! # fn main() {}
//! #
//! # fn write() -> error::Result<()> {
//!       // Setup the `Branches` struct.
//!       let mut master: Branch = Default::default();
//!       master.set_name("origin/master".to_string());
//!       master.set_interval("1m".to_string());
//!
//!       let mut feature_testing: Branch = Default::default();
//!       feature_testing.set_name("origin/feature/testing".to_string());
//!       feature_testing.set_interval("1m".to_string());
//!
//!       let repomon_branches = vec![master.clone(), feature_testing];
//!       let blah_branches = vec![master.clone()];
//!       let repomon_branches = vec![master];
//!
//!       let mut branch_map = BTreeMap::new();
//!       branch_map.insert("repomon".to_string(), repomon_branches.clone());
//!       branch_map.insert("repomon-config".to_string(), repomon_branches);
//!       branch_map.insert("blah".to_string(), blah_branches);
//!
//!       let mut branches: Branches = Default::default();
//!       branches.set_branch_map(branch_map);
//!
//!       // Write the TOML to the given buf.
//!       let mut buf = [0; 5000];
//!
//!       // Wrapped to drop mutable borrow.
//!       {
//!         let mut writer = Cursor::new(&mut buf[..]);
//!         write_toml(&branches, &mut writer)?;
//!       }
//!
//!       // Check that the result is the same as the TOML above.
//!       assert_eq!(TEST_TOML, String::from_utf8_lossy(&buf));
//! #   Ok(())
//! # }
//! ```
//!
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate bincode;
extern crate toml;

pub use config::{read_toml, write_toml, Branch, Branches};
pub use error::{Error, ErrorKind};
pub use message::Message;

mod config;
mod error;
mod message;

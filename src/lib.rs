// Copyright (c) 2017 repomon developers
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
//! # use repomon::{read_toml, Repomon};
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
//! # fn main() {
//! #     read().expect("")
//! # }
//! #
//! # fn read() -> error::Result<()> {
//!     let test_toml = r#"basedir = "/home/jozias/projects"
//!
//!     [[repos.ar2.remotes]]
//!     name = "origin"
//!     url = "jozias@jasonozias.com:repos/ar2.git"
//!
//!     [[repos.ar2.branch]]
//!     name = "master"
//!     interval = "1m"
//!     remotes = ["origin"]
//!
//!     [[repos.repomon.remotes]]
//!     name = "origin"
//!     url = "jozias@jasonozias.com:repos/repomon.git"
//!
//!     [[repos.repomon.remotes]]
//!     name = "gh"
//!     url = "git@github.com:rustyhorde/repomon.git"
//!
//!     [[repos.repomon.branch]]
//!     name = "master"
//!    interval = "1m"
//!     remotes = ["origin", "gh"]
//!
//!     [[repos.repomon.branch]]
//!     name = "feature/testing"
//!     interval = "1m"
//!     remotes = ["origin", "gh"]
//!     "#;
//!
//!     // Serialize the TOML above into a `Repomon` struct.
//!     let mut reader = Cursor::new(test_toml);
//!     let repomon = read_toml(&mut reader)?;
//!
//!     // Check the `Repomon` struct.
//!     let repos = repomon.repos();
//!     assert_eq!(repos.keys().len(), 2);
//!     assert!(repos.contains_key("repomon"));
//!     assert!(repos.contains_key("ar2"));
//!
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
//! # use repomon::{write_toml, Branch, Remote, Repo, Repomon};
//! # use std::collections::BTreeMap;
//! # use std::io::Cursor;
//! #
//! # mod error {
//! #     error_chain!{
//! #         foreign_links {
//! #             Io(::std::io::Error);
//! #             Repomon(::repomon::Error);
//! #             TomlDe(::toml::de::Error);
//! #             TomlSer(::toml::ser::Error);
//! #         }
//! #     }
//! # }
//! #
//! # const TEST_TOML: &str = r#"basedir = "/home/jozias/projects"
//! # [[repos.ar2.remotes]]
//! # name = "origin"
//! # url = "jozias@jasonozias.com:repos/ar2.git"
//! #
//! # [[repos.ar2.branch]]
//! # name = "master"
//! # interval = "1m"
//! # remotes = ["origin"]
//! # [[repos.repomon.remotes]]
//! # name = "origin"
//! # url = "jozias@jasonozias.com:repos/repomon.git"
//! #
//! # [[repos.repomon.remotes]]
//! # name = "gh"
//! # url = "git@github.com:rustyhorde/repomon.git"
//! #
//! # [[repos.repomon.branch]]
//! # name = "master"
//! # interval = "1m"
//! # remotes = ["origin", "gh"]
//! #
//! # [[repos.repomon.branch]]
//! # name = "feature/testing"
//! # interval = "1m"
//! # remotes = ["origin", "gh"]
//! # "#;
//! #
//! # fn main() {
//! #     write().expect("unable to write");
//! # }
//! #
//! # fn remotes() -> Vec<Remote> {
//! #     let mut origin: Remote = Default::default();
//! #     origin.set_name("origin".to_string());
//! #     origin.set_url("jozias@jasonozias.com:repos/repomon.git".to_string());
//! #
//! #     let mut github: Remote = Default::default();
//! #     github.set_name("gh".to_string());
//! #     github.set_url("git@github.com:rustyhorde/repomon.git".to_string());
//! #
//! #     vec![origin, github]
//! # }
//! #
//! # fn write() -> error::Result<()> {
//! #      // Setup the `Repomon` struct.
//! #      let remotes_to_monitor = vec!["origin", "gh"]
//! #          .iter()
//! #          .map(|x| x.to_string())
//! #          .collect::<Vec<String>>();
//! #
//! #      let mut master: Branch = Default::default();
//! #      master.set_name("master".to_string());
//! #      master.set_interval("1m".to_string());
//! #      master.set_remotes(remotes_to_monitor.clone());
//! #
//! #      let mut ar2_master: Branch = Default::default();
//! #      ar2_master.set_name("master".to_string());
//! #      ar2_master.set_interval("1m".to_string());
//! #      ar2_master.set_remotes(vec!["origin"].iter().map(|x| x.to_string()).collect());
//! #
//! #      let mut feature_testing: Branch = Default::default();
//! #      feature_testing.set_name("feature/testing".to_string());
//! #      feature_testing.set_interval("1m".to_string());
//! #      feature_testing.set_remotes(remotes_to_monitor);
//! #
//! #      let mut ar2_origin: Remote = Default::default();
//! #      ar2_origin.set_name("origin".to_string());
//! #      ar2_origin.set_url("jozias@jasonozias.com:repos/ar2.git".to_string());
//! #
//! #      let repomon_branches = vec![master, feature_testing];
//! #      let ar2_branches = vec![ar2_master];
//! #
//! #      let mut repomon_repo: Repo = Default::default();
//! #      repomon_repo.set_remotes(remotes());
//! #      repomon_repo.set_branch(repomon_branches);
//! #
//! #      let mut ar2_repo: Repo = Default::default();
//! #      ar2_repo.set_remotes(vec![ar2_origin]);
//! #      ar2_repo.set_branch(ar2_branches);
//! #
//! #      let mut repo_map = BTreeMap::new();
//! #      repo_map.insert("ar2".to_string(), ar2_repo);
//! #      repo_map.insert("repomon".to_string(), repomon_repo);
//! #
//!       let mut repomon: Repomon = Default::default();
//!       repomon.set_basedir("/home/jozias/projects".to_string());
//!       repomon.set_repos(repo_map);
//!
//!       // Write the TOML to the given buf.
//!       let mut buf = [0; 5000];
//!
//!       // Wrapped to drop mutable borrow.
//!       {
//!         let mut writer = Cursor::new(&mut buf[..]);
//!         write_toml(&repomon, &mut writer)?;
//!       }
//!
//!       // Check that the result is the same as the TOML above.
//!       let filtered = buf.iter().filter(|x| **x > 0).cloned().collect::<Vec<u8>>();
//!       assert_eq!(
//!           TEST_TOML,
//!           String::from_utf8(filtered).expect("Invalid UTF-8 in result")
//!       );
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
extern crate url;
extern crate uuid;

pub use config::{read_toml, write_toml, Branch, Remote, Repo, Repomon};
pub use error::{Error, ErrorKind};
pub use message::Message;

mod config;
mod error;
mod message;

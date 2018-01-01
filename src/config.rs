// Copyright (c) 2017 repomon developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Configuration Management for repomon
use error::Result;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{Read, Write};
use toml;

/// A map of repo name to branch definitions.
#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize, Setters)]
pub struct Branches {
    /// A map of repo name to a vector branches to monitor.
    #[get = "pub"]
    #[set = "pub"]
    #[serde(rename = "branch")]
    branch_map: BTreeMap<String, Vec<Branch>>,
}

impl fmt::Display for Branches {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Branches {{")?;

        for (repo, branches) in &self.branch_map {
            for branch in branches {
                writeln!(fmt, "    {} -- {}", repo, branch)?;
            }
        }

        write!(fmt, "}}")
    }
}

/// A branch to monitor for changes.
#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize, Setters)]
pub struct Branch {
    /// The fully qualified branch name, e.g. `origin/master` for the remote
    /// or `master` for the local.
    #[get = "pub"]
    #[set = "pub"]
    name: String,
    /// The interval to check the branch for changes.
    #[get = "pub"]
    #[set = "pub"]
    interval: String,
}

impl fmt::Display for Branch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name)
    }
}

/// Read TOML from the given `reader` and deserialize into a `Branches` struct.
pub fn read_toml<R>(reader: &mut R) -> Result<Branches>
where
    R: Read,
{
    let mut toml_str = String::new();
    let bytes_read = reader.read_to_string(&mut toml_str)?;

    if bytes_read > 0 {
        Ok(toml::from_str(&toml_str)?)
    } else {
        Err("Unable to read any bytes from the reader".into())
    }
}

/// Write TOML serialized from the `Branches` struct to the given `writer`.
pub fn write_toml<W>(repos: &Branches, writer: &mut W) -> Result<()>
where
    W: Write,
{
    let toml = toml::to_string(&repos)?;
    Ok(writer.write_all(toml.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use super::{Branch, Branches};
    use std::collections::BTreeMap;
    use std::io::Cursor;
    use toml;

    const TEST_TOML: &str = r#"[[branch.blah]]
name = "origin/master"
interval = "1m"

[[branch.repomon]]
name = "origin/master"
interval = "1m"

[[branch.repomon]]
name = "origin/feature/testing"
interval = "1m"

[[branch.repomon-config]]
name = "origin/master"
interval = "1m"
"#;

    fn setup_branches() -> Branches {
        let master = Branch {
            name: "origin/master".to_string(),
            interval: "1m".to_string(),
        };

        let feature_testing = Branch {
            name: "origin/feature/testing".to_string(),
            interval: "1m".to_string(),
        };

        let repomon_branches = vec![master.clone(), feature_testing];
        let blah_branches = vec![master.clone()];
        let repomon_config_branches = vec![master];

        let mut branch_map = BTreeMap::new();
        branch_map.insert("repomon".to_string(), repomon_branches);
        branch_map.insert("repomon-config".to_string(), repomon_config_branches);
        branch_map.insert("blah".to_string(), blah_branches);

        Branches {
            branch_map: branch_map,
        }
    }

    fn test_branches(branches: &Branches) {
        let branch_map = branches.branch_map();
        assert_eq!(branch_map.keys().len(), 3);
        assert!(branch_map.contains_key("repomon"));
        assert!(branch_map.contains_key("repomon-config"));
        assert!(branch_map.contains_key("blah"));

        let mut branches = branch_map
            .get("repomon")
            .ok_or("invalid key")
            .expect("Unable to lookup repomon repo");
        assert_eq!(branches.len(), 2);
        branches = branch_map
            .get("repomon-config")
            .ok_or("invalid key")
            .expect("Unable to lookup repomon repo");
        assert_eq!(branches.len(), 1);
        branches = branch_map
            .get("blah")
            .ok_or("invalid key")
            .expect("Unable to lookup repomon repo");
        assert_eq!(branches.len(), 1);
    }

    #[test]
    fn serialize() {
        let branches = setup_branches();
        let toml = toml::to_string(&branches).expect("Unable to serialize to TOML");
        assert_eq!(TEST_TOML, toml);
    }

    #[test]
    fn deserialize() {
        let branches: Branches = toml::from_str(TEST_TOML).expect("Unable to deserialize TOML");
        test_branches(&branches);
    }

    #[test]
    fn empty_reader() {
        let mut cursor = Cursor::new(vec![]);
        match super::read_toml(&mut cursor) {
            Ok(_) => assert!(false, "0 bytes read should error"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn read_toml() {
        let mut reader = Cursor::new(TEST_TOML);

        match super::read_toml(&mut reader) {
            Ok(branches) => test_branches(&branches),
            Err(_) => assert!(false, "Unable to parse TOML"),
        }
    }

    #[test]
    fn write_toml() {
        let mut buf = [0; 5000];
        let branches = setup_branches();
        {
            let mut writer = Cursor::new(&mut buf[..]);
            match super::write_toml(&branches, &mut writer) {
                Ok(_) => {}
                Err(_) => assert!(false, "Unable to write TOML"),
            }
        }

        let filtered = buf.iter().filter(|x| **x > 0).cloned().collect::<Vec<u8>>();
        assert_eq!(
            TEST_TOML,
            String::from_utf8(filtered).expect("Invalid UTF-8 in result")
        );
    }
}

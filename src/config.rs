// Copyright (c) 2017 repomon developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Configuration Management for repomon
use error::Result;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{Read, Write};
use toml;

/// The base repomon config.
#[derive(Clone, Debug, Default, Deserialize, Getters, PartialEq, Serialize, Setters)]
pub struct Repomon {
    /// The base directory to look for repositories.
    #[get = "pub"]
    #[set = "pub"]
    basedir: String,
    /// A map of repository name to repository definitions.
    #[get = "pub"]
    #[set = "pub"]
    repos: BTreeMap<String, Repo>,
}

impl fmt::Display for Repomon {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Repomon {{")?;

        for (repo_name, repo) in &self.repos {
            writeln!(fmt, "    {} -- {}", repo_name, repo)?;
        }

        write!(fmt, "}}")
    }
}

/// A repomon repository definition
#[derive(Clone, Debug, Default, Deserialize, Getters, PartialEq, Serialize, Setters)]
pub struct Repo {
    /// The repository remotes for branch comparison.
    #[get = "pub"]
    #[set = "pub"]
    remotes: Vec<Remote>,
    /// The repository branches to monitor.
    #[get = "pub"]
    #[set = "pub"]
    branch: Vec<Branch>,
}

impl fmt::Display for Repo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Repo {{")?;
        for remote in &self.remotes {
            write!(fmt, "    {}", remote)?;
        }

        for branch in &self.branch {
            write!(fmt, "    {}", branch)?;
        }
        write!(fmt, "}}")
    }
}

/// A branch to monitor for changes.
#[derive(Clone, Debug, Default, Deserialize, Getters, PartialEq, Serialize, Setters)]
pub struct Branch {
    /// The branch name, i.e. 'master'
    #[get = "pub"]
    #[set = "pub"]
    name: String,
    /// The interval to check the branch for changes.
    #[get = "pub"]
    #[set = "pub"]
    interval: String,
    /// The list of remotes to check this branch against.
    #[get = "pub"]
    #[set = "pub"]
    remotes: Vec<String>,
}

impl Branch {
    /// Convert an interval to milliseconds
    pub fn interval_to_ms(&self) -> Result<usize> {
        let interval_re = Regex::new(r"^(\d+)(s|m|h|d)$")?;
        if interval_re.is_match(&self.interval) {
            if let Some(caps) = interval_re.captures(&self.interval) {
                if caps.len() != 3 {
                    return Err(format!("invalid branch interval: {}", self.interval).into());
                }

                let units = caps.get(2).map_or("", |m| m.as_str());
                let value = caps.get(1).map_or("", |m| m.as_str()).parse::<usize>()?;

                let factor = match units {
                    "s" => 1000,
                    "m" => 60_000,
                    "h" => 3_600_000,
                    "d" => 86_400_000,
                    _ => return Err(format!("invalid branch interval: {}", self.interval).into()),
                };

                Ok(value * factor)
            } else {
                return Err(format!("invalid branch interval: {}", self.interval).into());
            }
        } else {
            Err(format!("invalid branch interval: {}", self.interval).into())
        }
    }
}

impl fmt::Display for Branch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.name)
    }
}

/// A remote to check a branch against
#[derive(Clone, Debug, Default, Deserialize, Getters, PartialEq, Serialize, Setters)]
pub struct Remote {
    /// The remote name, i.e. 'origin'
    #[get = "pub"]
    #[set = "pub"]
    name: String,
    /// The remote url
    #[get = "pub"]
    #[set = "pub"]
    url: String,
}

impl fmt::Display for Remote {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.name, self.url)
    }
}

/// Read TOML from the given `reader` and deserialize into a `Repomon` struct.
pub fn read_toml<R>(reader: &mut R) -> Result<Repomon>
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

/// Write TOML serialized from the `Repomon` struct to the given `writer`.
pub fn write_toml<W>(repos: &Repomon, writer: &mut W) -> Result<()>
where
    W: Write,
{
    let toml = toml::to_string(&repos)?;
    Ok(writer.write_all(toml.as_bytes())?)
}

#[cfg(test)]
mod tests {
    use super::{Branch, Remote, Repo, Repomon};
    use std::collections::BTreeMap;
    use std::io::Cursor;
    use toml;

    const TEST_TOML: &str = r#"basedir = "/home/jozias/projects"
[[repos.ar2.remotes]]
name = "origin"
url = "jozias@jasonozias.com:repos/ar2.git"

[[repos.ar2.branch]]
name = "master"
interval = "1m"
remotes = ["origin"]
[[repos.repomon.remotes]]
name = "origin"
url = "jozias@jasonozias.com:repos/repomon.git"

[[repos.repomon.remotes]]
name = "gh"
url = "git@github.com:rustyhorde/repomon.git"

[[repos.repomon.branch]]
name = "master"
interval = "1m"
remotes = ["origin", "gh"]

[[repos.repomon.branch]]
name = "feature/testing"
interval = "1m"
remotes = ["origin", "gh"]
"#;

    fn remotes() -> Vec<Remote> {
        let mut origin: Remote = Default::default();
        origin.set_name("origin".to_string());
        origin.set_url("jozias@jasonozias.com:repos/repomon.git".to_string());

        let mut github: Remote = Default::default();
        github.set_name("gh".to_string());
        github.set_url("git@github.com:rustyhorde/repomon.git".to_string());

        vec![origin, github]
    }

    fn setup_repomon() -> Repomon {
        let remotes_to_monitor = vec!["origin", "gh"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let master = Branch {
            name: "master".to_string(),
            interval: "1m".to_string(),
            remotes: remotes_to_monitor.clone(),
        };

        let ar2_master = Branch {
            name: "master".to_string(),
            interval: "1m".to_string(),
            remotes: vec!["origin"].iter().map(|x| x.to_string()).collect(),
        };

        let feature_testing = Branch {
            name: "feature/testing".to_string(),
            interval: "1m".to_string(),
            remotes: remotes_to_monitor,
        };

        let mut ar2_origin: Remote = Default::default();
        ar2_origin.set_name("origin".to_string());
        ar2_origin.set_url("jozias@jasonozias.com:repos/ar2.git".to_string());

        let repomon_branches = vec![master, feature_testing];
        let ar2_branches = vec![ar2_master];

        let repomon_repo = Repo {
            remotes: remotes(),
            branch: repomon_branches,
        };

        let ar2_repo = Repo {
            remotes: vec![ar2_origin],
            branch: ar2_branches,
        };

        let mut repo_map = BTreeMap::new();
        repo_map.insert("ar2".to_string(), ar2_repo);
        repo_map.insert("repomon".to_string(), repomon_repo);

        Repomon {
            basedir: "/home/jozias/projects".to_string(),
            repos: repo_map,
        }
    }

    fn test_repomon(repomon: &Repomon) {
        let repo_map = repomon.repos();
        assert_eq!(repo_map.keys().len(), 2);
        assert!(repo_map.contains_key("repomon"));
        assert!(repo_map.contains_key("ar2"));

        let repomon = repo_map
            .get("repomon")
            .ok_or("invalid key")
            .expect("Unable to lookup repomon repo");
        assert_eq!(repomon.remotes().len(), 2);
        assert_eq!(repomon.branch().len(), 2);

        let ar2 = repo_map
            .get("ar2")
            .ok_or("invalid key")
            .expect("Unable to lookup ar2 repo");
        assert_eq!(ar2.remotes().len(), 1);
        assert_eq!(ar2.branch().len(), 1);
    }

    #[test]
    fn serialize() {
        let repomon = setup_repomon();
        let toml = toml::to_string(&repomon).expect("Unable to serialize to TOML");
        assert_eq!(TEST_TOML, toml);
    }

    #[test]
    fn deserialize() {
        let repomon: Repomon = toml::from_str(TEST_TOML).expect("Unable to deserialize TOML");
        assert_eq!(repomon.basedir(), "/home/jozias/projects");
        test_repomon(&repomon);
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
            Ok(repomon) => test_repomon(&repomon),
            Err(_) => assert!(false, "Unable to parse TOML"),
        }
    }

    #[test]
    fn write_toml() {
        let mut buf = [0; 5000];
        let repomon = setup_repomon();
        {
            let mut writer = Cursor::new(&mut buf[..]);
            match super::write_toml(&repomon, &mut writer) {
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

    fn check_ms_result(expected: usize, branch: &Branch) {
        if let Ok(ms) = branch.interval_to_ms() {
            assert_eq!(expected, ms);
        } else {
            assert!(false, "invalid branch interval");
        }
    }

    #[test]
    fn interval_to_ms() {
        let mut branch: Branch = Default::default();
        branch.set_interval("1s".to_string());
        check_ms_result(1000, &branch);
        branch.set_interval("5s".to_string());
        check_ms_result(5000, &branch);
        branch.set_interval("1m".to_string());
        check_ms_result(60_000, &branch);
        branch.set_interval("5m".to_string());
        check_ms_result(300_000, &branch);
        branch.set_interval("1h".to_string());
        check_ms_result(3_600_000, &branch);
        branch.set_interval("5h".to_string());
        check_ms_result(18_000_000, &branch);
        branch.set_interval("1d".to_string());
        check_ms_result(86_400_000, &branch);
        branch.set_interval("5d".to_string());
        check_ms_result(432_000_000, &branch);
    }
}

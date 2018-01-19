// Copyright (c) 2017 repomon developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! repomon messages
use config::{Branch, Remote};
use std::collections::BTreeMap;
use std::fmt;
use uuid::Uuid;

/// Struct sent via tx to clients;
#[derive(Clone, Debug, Default, Deserialize, Getters, MutGetters, Serialize, Setters)]
pub struct Message {
    /// The unique message identifier.
    #[get = "pub"]
    #[set = "pub"]
    uuid: Uuid,
    /// The repo name.
    #[get = "pub"]
    #[set = "pub"]
    repo: String,
    /// The messages per branch/remote combo.
    #[get = "pub"]
    #[set = "pub"]
    messages: BTreeMap<Branch, BTreeMap<Remote, String>>,
}

impl fmt::Display for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (ref branch, ref remotes) in &self.messages {
            let len = remotes.len();
            for (idx, (remote, message)) in remotes.iter().enumerate() {
                write!(
                    fmt,
                    "{}: {}/{} ({}) - {}",
                    self.uuid,
                    self.repo,
                    branch.name(),
                    remote.name(),
                    message
                )?;

                if idx < len - 1 {
                    write!(fmt, "\n")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use bincode::{deserialize, serialize, Infinite};
    use config::{Branch, Remote};
    use message::Message;
    use std::collections::BTreeMap;
    use uuid::{self, Uuid};

    const MSG_BYTES: [u8; 501] = [
        36, 0, 0, 0, 0, 0, 0, 0, 98, 52, 50, 56, 98, 53, 100, 57, 45, 100, 102, 49, 57, 45, 53, 98,
        98, 57, 45, 97, 49, 100, 99, 45, 49, 49, 53, 101, 48, 55, 49, 98, 56, 51, 54, 99, 7, 0, 0,
        0, 0, 0, 0, 0, 114, 101, 112, 111, 109, 111, 110, 2, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0,
        0, 0, 0, 102, 101, 97, 116, 117, 114, 101, 47, 116, 101, 115, 116, 2, 0, 0, 0, 0, 0, 0, 0,
        49, 109, 2, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 111, 114, 105, 103, 105, 110, 2,
        0, 0, 0, 0, 0, 0, 0, 103, 104, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 103, 104, 0,
        0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 89, 111, 117, 114, 32, 98, 114, 97, 110, 99,
        104, 32, 105, 115, 32, 117, 112, 32, 116, 111, 32, 100, 97, 116, 101, 32, 119, 105, 116,
        104, 32, 39, 103, 104, 47, 102, 101, 97, 116, 117, 114, 101, 47, 116, 101, 115, 116, 39, 6,
        0, 0, 0, 0, 0, 0, 0, 111, 114, 105, 103, 105, 110, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0,
        0, 0, 0, 89, 111, 117, 114, 32, 98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 117, 112, 32,
        116, 111, 32, 100, 97, 116, 101, 32, 119, 105, 116, 104, 32, 39, 111, 114, 105, 103, 105,
        110, 47, 102, 101, 97, 116, 117, 114, 101, 47, 116, 101, 115, 116, 39, 6, 0, 0, 0, 0, 0, 0,
        0, 109, 97, 115, 116, 101, 114, 2, 0, 0, 0, 0, 0, 0, 0, 49, 109, 2, 0, 0, 0, 0, 0, 0, 0, 6,
        0, 0, 0, 0, 0, 0, 0, 111, 114, 105, 103, 105, 110, 2, 0, 0, 0, 0, 0, 0, 0, 103, 104, 2, 0,
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 103, 104, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
        0, 0, 0, 89, 111, 117, 114, 32, 98, 114, 97, 110, 99, 104, 32, 105, 115, 32, 117, 112, 32,
        116, 111, 32, 100, 97, 116, 101, 32, 119, 105, 116, 104, 32, 39, 103, 104, 47, 109, 97,
        115, 116, 101, 114, 39, 6, 0, 0, 0, 0, 0, 0, 0, 111, 114, 105, 103, 105, 110, 0, 0, 0, 0,
        0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 89, 111, 117, 114, 32, 98, 114, 97, 110, 99, 104, 32,
        105, 115, 32, 117, 112, 32, 116, 111, 32, 100, 97, 116, 101, 32, 119, 105, 116, 104, 32,
        39, 111, 114, 105, 103, 105, 110, 47, 109, 97, 115, 116, 101, 114, 39,
    ];

    #[test]
    fn serialize_message() {
        let mut message: Message = Default::default();

        let mut origin: Remote = Default::default();
        origin.set_name("origin".to_string());

        let mut gh: Remote = Default::default();
        gh.set_name("gh".to_string());

        let mut master_remote_messages = BTreeMap::new();
        master_remote_messages.insert(
            origin.clone(),
            "Your branch is up to date with 'origin/master'".to_string(),
        );
        master_remote_messages.insert(
            gh.clone(),
            "Your branch is up to date with 'gh/master'".to_string(),
        );

        let mut feature_remote_messages = BTreeMap::new();
        feature_remote_messages.insert(
            origin,
            "Your branch is up to date with 'origin/feature/test'".to_string(),
        );
        feature_remote_messages.insert(
            gh,
            "Your branch is up to date with 'gh/feature/test'".to_string(),
        );

        let mut master_branch: Branch = Default::default();
        master_branch.set_name("master".to_string());
        master_branch.set_interval("1m".to_string());
        master_branch.set_remotes(vec!["origin", "gh"].iter().map(|x| x.to_string()).collect());

        let mut feature_branch: Branch = Default::default();
        feature_branch.set_name("feature/test".to_string());
        feature_branch.set_interval("1m".to_string());
        feature_branch.set_remotes(vec!["origin", "gh"].iter().map(|x| x.to_string()).collect());

        let mut messages = BTreeMap::new();
        messages.insert(master_branch, master_remote_messages);
        messages.insert(feature_branch, feature_remote_messages);

        message.set_uuid(Uuid::new_v5(&uuid::NAMESPACE_OID, "test"));
        message.set_repo("repomon".to_string());
        message.set_messages(messages);

        let msg_bytes = serialize(&message, Infinite).expect("unable to serialize message");
        let mut expected: Vec<u8> = Vec::new();
        expected.extend(MSG_BYTES.iter());
        assert_eq!(msg_bytes, expected);
    }

    #[test]
    fn deserialize_bytes() {
        let message: Message = deserialize(&MSG_BYTES).expect("unable to deserialize message");
        assert_eq!(
            message.uuid().to_string(),
            "b428b5d9-df19-5bb9-a1dc-115e071b836c"
        );
        assert_eq!(message.repo(), "repomon");
        assert_eq!(message.messages().len(), 2);

        for (idx, (branch, remotes)) in message.messages().iter().enumerate() {
            match idx {
                0 => {
                    assert_eq!(branch.name(), "feature/test");
                    assert_eq!(branch.interval(), "1m");
                    assert_eq!(branch.remotes(), &["origin", "gh"]);

                    for (jdx, (remote, message)) in remotes.iter().enumerate() {
                        match jdx {
                            0 => {
                                assert_eq!(remote.name(), "gh");
                                assert_eq!(
                                    message,
                                    "Your branch is up to date with 'gh/feature/test'"
                                );
                            }
                            1 => {
                                assert_eq!(remote.name(), "origin");
                                assert_eq!(
                                    message,
                                    "Your branch is up to date with 'origin/feature/test'"
                                );
                            }
                            _ => assert!(false),
                        }
                    }
                }
                1 => {
                    assert_eq!(branch.name(), "master");
                    assert_eq!(branch.interval(), "1m");
                    assert_eq!(branch.remotes(), &["origin", "gh"]);

                    for (jdx, (remote, message)) in remotes.iter().enumerate() {
                        match jdx {
                            0 => {
                                assert_eq!(remote.name(), "gh");
                                assert_eq!(message, "Your branch is up to date with 'gh/master'");
                            }
                            1 => {
                                assert_eq!(remote.name(), "origin");
                                assert_eq!(
                                    message,
                                    "Your branch is up to date with 'origin/master'"
                                );
                            }
                            _ => assert!(false),
                        }
                    }
                }
                _ => assert!(false),
            }
        }
    }
}

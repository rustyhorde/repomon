// Copyright (c) 2017 repomon developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! repomon messages
use config::Branch;
use std::fmt;

/// Struct sent via tx to clients;
#[derive(Clone, Debug, Default, Deserialize, Getters, Serialize, Setters)]
pub struct Message {
    /// The repo name.
    #[get = "pub"]
    #[set = "pub"]
    repo: String,
    /// The branch that generated this message.
    #[get = "pub"]
    #[set = "pub"]
    branch: Branch,
    /// The current count.
    #[get = "pub"]
    #[set = "pub"]
    count: usize,
}

impl fmt::Display for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} -- {}: {}", self.repo, self.branch, self.count)
    }
}

#[cfg(test)]
mod test {
    use bincode::{deserialize, serialize, Infinite};
    use config::Branch;
    use message::Message;

    const MSG_BYTES: [u8; 52] = [
        7, 0, 0, 0, 0, 0, 0, 0, 114, 101, 112, 111, 109, 111, 110, 13, 0, 0, 0, 0, 0, 0, 0, 111,
        114, 105, 103, 105, 110, 47, 109, 97, 115, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    #[test]
    fn serialize_message() {
        let mut message: Message = Default::default();
        let mut branch: Branch = Default::default();
        branch.set_name("origin/master".to_string());
        message.set_repo("repomon".to_string());
        message.set_branch(branch);
        message.set_count(0);

        let msg_bytes = serialize(&message, Infinite).expect("unable to serialize message");
        let mut expected = Vec::new();
        expected.extend(MSG_BYTES.iter());
        assert_eq!(msg_bytes, expected);
    }

    #[test]
    fn deserialize_bytes() {
        let message: Message = deserialize(&MSG_BYTES).expect("unable to deserialize message");
        assert_eq!(message.repo(), "repomon");
        assert_eq!(message.branch().name(), "origin/master");
        assert_eq!(*message.count(), 0);
    }
}

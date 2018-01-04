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
    /// The branch that generated this message.
    #[get = "pub"]
    #[set = "pub"]
    branch: Branch,
    /// The current count.
    #[get = "pub"]
    #[get_mut = "pub"]
    #[set = "pub"]
    count: usize,
}

impl fmt::Display for Message {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {} -- {}", self.uuid, self.repo, self.branch)
    }
}

#[cfg(test)]
mod test {
    use bincode::{deserialize, serialize, Infinite};
    use config::Branch;
    use message::Message;
    use uuid::{self, Uuid};

    const MSG_BYTES: [u8; 123] = [
        36, 0, 0, 0, 0, 0, 0, 0, 98, 52, 50, 56, 98, 53, 100, 57, 45, 100, 102, 49, 57, 45, 53, 98,
        98, 57, 45, 97, 49, 100, 99, 45, 49, 49, 53, 101, 48, 55, 49, 98, 56, 51, 54, 99, 7, 0, 0,
        0, 0, 0, 0, 0, 114, 101, 112, 111, 109, 111, 110, 6, 0, 0, 0, 0, 0, 0, 0, 109, 97, 115,
        116, 101, 114, 2, 0, 0, 0, 0, 0, 0, 0, 49, 109, 2, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
        0, 0, 111, 114, 105, 103, 105, 110, 2, 0, 0, 0, 0, 0, 0, 0, 103, 104, 0, 0, 0, 0, 0, 0, 0,
        0,
    ];

    #[test]
    fn serialize_message() {
        let mut message: Message = Default::default();
        let mut branch: Branch = Default::default();
        branch.set_name("master".to_string());
        branch.set_interval("1m".to_string());
        branch.set_remotes(vec!["origin", "gh"].iter().map(|x| x.to_string()).collect());
        message.set_uuid(Uuid::new_v5(&uuid::NAMESPACE_OID, "test"));
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
        assert_eq!(
            message.uuid().to_string(),
            "b428b5d9-df19-5bb9-a1dc-115e071b836c"
        );
        assert_eq!(message.repo(), "repomon");
        assert_eq!(message.branch().name(), "master");
        assert_eq!(message.branch().remotes(), &vec!["origin", "gh"]);
        assert_eq!(message.branch().interval(), "1m");
        assert_eq!(*message.count(), 0);
    }
}

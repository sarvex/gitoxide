use crate::config;
use crate::config::tree::{keys, Key, Remote, Section};

impl Remote {
    /// The `remote.pushDefault` key
    pub const PUSH_DEFAULT: keys::RemoteName = keys::RemoteName::new_remote_name("pushDefault", &config::Tree::REMOTE);
}

impl Section for Remote {
    fn name(&self) -> &str {
        "remote"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::PUSH_DEFAULT]
    }
}

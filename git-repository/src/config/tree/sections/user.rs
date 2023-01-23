use crate::config;
use crate::config::tree::{keys, Key, Section, User};

impl User {
    /// The `user.name` key
    pub const NAME: keys::Any = keys::Any::new("name", &config::Tree::USER);
    /// The `user.email` key
    pub const EMAIL: keys::Any = keys::Any::new("email", &config::Tree::USER);
}

impl Section for User {
    fn name(&self) -> &str {
        "user"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::NAME, &Self::EMAIL]
    }
}

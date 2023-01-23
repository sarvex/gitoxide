pub(crate) mod root {
    use super::sections;
    use crate::config::tree::Section;

    /// The root of the configuration tree, suitable to discover all sub-sections at runtime or compile time.
    #[derive(Copy, Clone, Default)]
    pub struct Tree;

    impl Tree {
        /// The `author` section.
        pub const AUTHOR: sections::Author = sections::Author;
        /// The `branch` section.
        pub const BRANCH: sections::Branch = sections::Branch;
        /// The `checkout` section.
        pub const CHECKOUT: sections::Checkout = sections::Checkout;
        /// The `commiter` section.
        pub const COMMITTER: sections::Committer = sections::Committer;
        /// The `gitoxide` section.
        pub const GITOXIDE: sections::Gitoxide = sections::Gitoxide;
        /// The `remote` section.
        pub const REMOTE: sections::Remote = sections::Remote;
        /// The `user` section.
        pub const USER: sections::User = sections::User;

        /// List all available sections.
        pub fn sections(&self) -> &[&dyn Section] {
            &[
                &Self::AUTHOR,
                &Self::BRANCH,
                &Self::CHECKOUT,
                &Self::COMMITTER,
                &Self::GITOXIDE,
                &Self::REMOTE,
                &Self::USER,
            ]
        }
    }
}

mod sections;
pub use sections::{branch, checkout, gitoxide, Author, Branch, Checkout, Committer, Gitoxide, Remote, User};

/// Generic value implementations for static instantiation.
pub mod keys;

mod traits;
pub use traits::{Key, Link, Section, SubSectionRequirement};

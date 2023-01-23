use crate::config::tree::{Gitoxide, Key, Section};

impl Gitoxide {
    /// The `gitoxide.author` section.
    pub const AUTHOR: Author = Author;
    /// The `gitoxide.commit` section.
    pub const COMMIT: Commit = Commit;
    /// The `gitoxide.committer` section.
    pub const COMMITTER: Committer = Committer;
}

impl Section for Gitoxide {
    fn name(&self) -> &str {
        "gitoxide"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[]
    }

    fn sub_sections(&self) -> &[&dyn Section] {
        &[&Self::AUTHOR, &Self::COMMIT, &Self::COMMITTER]
    }
}

mod subsections {
    use crate::config::tree::{keys, Gitoxide, Key, Section};
    use crate::config::Tree;

    /// The `author` sub-section.
    #[derive(Copy, Clone, Default)]
    pub struct Author;

    impl Author {
        /// The `gitoxide.author.nameFallback` key.
        pub const NAME_FALLBACK: keys::Any =
            keys::Any::new("nameFallback", &Gitoxide::AUTHOR).with_environment_override("GIT_AUTHOR_NAME");
        /// The `gitoxide.author.emailFallback` key.
        pub const EMAIL_FALLBACK: keys::Any =
            keys::Any::new("emailFallback", &Gitoxide::AUTHOR).with_environment_override("GIT_AUTHOR_EMAIL");
    }

    impl Section for Author {
        fn name(&self) -> &str {
            "author"
        }

        fn keys(&self) -> &[&dyn Key] {
            &[&Self::NAME_FALLBACK, &Self::EMAIL_FALLBACK]
        }

        fn parent(&self) -> Option<&dyn Section> {
            Some(&Tree::GITOXIDE)
        }
    }

    /// The `committer` sub-section.
    #[derive(Copy, Clone, Default)]
    pub struct Committer;

    impl Committer {
        /// The `gitoxide.committer.nameFallback` key.
        pub const NAME_FALLBACK: keys::Any =
            keys::Any::new("nameFallback", &Gitoxide::COMMITTER).with_environment_override("GIT_COMMITTER_NAME");
        /// The `gitoxide.committer.emailFallback` key.
        pub const EMAIL_FALLBACK: keys::Any =
            keys::Any::new("emailFallback", &Gitoxide::COMMITTER).with_environment_override("GIT_COMMITTER_EMAIL");
    }

    impl Section for Committer {
        fn name(&self) -> &str {
            "committer"
        }

        fn keys(&self) -> &[&dyn Key] {
            &[&Self::NAME_FALLBACK, &Self::EMAIL_FALLBACK]
        }

        fn parent(&self) -> Option<&dyn Section> {
            Some(&Tree::GITOXIDE)
        }
    }

    /// The `commit` sub-section.
    #[derive(Copy, Clone, Default)]
    pub struct Commit;

    impl Commit {
        /// The `gitoxide.commit.authorDate` key.
        pub const AUTHOR_DATE: keys::Time =
            keys::Time::new_time("authorDate", &Gitoxide::COMMIT).with_environment_override("GIT_AUTHOR_DATE");
        /// The `gitoxide.commit.committerDate` key.
        pub const COMMITTER_DATE: keys::Time =
            keys::Time::new_time("committerDate", &Gitoxide::COMMIT).with_environment_override("GIT_COMMITTER_DATE");
    }

    impl Section for Commit {
        fn name(&self) -> &str {
            "commit"
        }

        fn keys(&self) -> &[&dyn Key] {
            &[]
        }

        fn parent(&self) -> Option<&dyn Section> {
            Some(&Tree::GITOXIDE)
        }
    }
}
pub use subsections::{Author, Commit, Committer};

mod tree {
    use git_object::bstr::BStr;
    use std::borrow::Cow;

    fn bcow(input: &str) -> Cow<'_, BStr> {
        Cow::Borrowed(input.into())
    }

    mod keys {
        use git_repository as git;
        use git_repository::config::tree::{Key, Section};

        #[test]
        fn any() {
            assert!(
                git::config::Tree.sections().len() > 0,
                "the root has at least one section"
            );
            assert_eq!(git::config::Tree::AUTHOR.name(), "author");
            assert_eq!(git::config::tree::Author.keys().len(), 2);
            assert_eq!(git::config::tree::Author::NAME.name(), "name");
            assert_eq!(git::config::tree::Author::EMAIL.name(), "email");
            assert_eq!(
                git::config::tree::Author::NAME
                    .validated_assignment("user".into())
                    .unwrap(),
                "author.name=user"
            );
            assert_eq!(
                git::config::tree::Author::NAME
                    .validated_assignment("user".into())
                    .unwrap(),
                "author.name=user"
            );
        }

        #[test]
        fn remote_name() {
            assert!(git::config::tree::Remote::PUSH_DEFAULT
                .validate("origin".into())
                .is_ok());
            assert!(git::config::tree::Remote::PUSH_DEFAULT
                .validate("https://github.com/byron/gitoxide".into())
                .is_ok());
        }
    }

    mod branch {
        use crate::config::tree::bcow;
        use git_repository::config::tree::{branch, Branch, Key};

        #[test]
        fn merge() {
            assert!(branch::Merge::try_into_fullrefname(bcow("refs/heads/main")).is_ok());
            assert!(branch::Merge::try_into_fullrefname(bcow("main")).is_err());

            assert!(Branch::MERGE.full_name(None).is_err());
            assert_eq!(
                Branch::MERGE.full_name(Some("name".into())).expect("valid"),
                "branch.name.merge"
            );
        }
    }

    mod checkout {
        // TODO: tests
    }

    mod gitoxide {
        mod commit {
            use git_repository::config::tree::{gitoxide, Key};

            #[test]
            fn author_and_committer_date() {
                assert_eq!(
                    gitoxide::Commit::AUTHOR_DATE
                        .validated_assignment("Thu, 1 Aug 2022 12:45:06 +0800".into())
                        .expect("valid"),
                    "gitoxide.commit.authorDate=Thu, 1 Aug 2022 12:45:06 +0800"
                );
                assert_eq!(
                    gitoxide::Commit::COMMITTER_DATE
                        .validated_assignment("Thu, 1 Aug 2022 12:45:06 +0800".into())
                        .expect("valid"),
                    "gitoxide.commit.committerDate=Thu, 1 Aug 2022 12:45:06 +0800"
                );
            }
        }
        mod author {
            use git_repository::config::tree::{gitoxide, Key};

            #[test]
            fn name_and_email_fallback() {
                assert_eq!(
                    gitoxide::Author::NAME_FALLBACK
                        .validated_assignment("name".into())
                        .expect("valid"),
                    "gitoxide.author.nameFallback=name"
                );
                assert_eq!(
                    gitoxide::Author::EMAIL_FALLBACK
                        .validated_assignment("email".into())
                        .expect("valid"),
                    "gitoxide.author.emailFallback=email"
                );
            }
        }
        mod committer {
            use git_repository::config::tree::{gitoxide, Key};

            #[test]
            fn name_and_email_fallback() {
                assert_eq!(
                    gitoxide::Committer::NAME_FALLBACK
                        .validated_assignment("name".into())
                        .expect("valid"),
                    "gitoxide.committer.nameFallback=name"
                );
                assert_eq!(
                    gitoxide::Committer::EMAIL_FALLBACK
                        .validated_assignment("email".into())
                        .expect("valid"),
                    "gitoxide.committer.emailFallback=email"
                );
            }
        }
    }
}

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use bstr::BString;

pub struct Pattern {
    /// the actual pattern bytes
    _inner: BString,
    _mode: pattern::Mode,
}

pub mod pattern {
    use crate::Pattern;
    use bitflags::bitflags;
    use bstr::{BStr, BString};

    bitflags! {
        #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
        pub struct Mode: u32 {
            /// The pattern does not contain a sub-directory and - it doesn't contain slashes after removing the trailing one.
            const NO_SUB_DIR = 1 << 0;
            /// A pattern that is '*literal', meaning that it ends with what's given here
            const ENDS_WITH = 1 << 1;
            /// The pattern must match a directory, and not a file.
            const MUST_BE_DIR = 1 << 2;
            const NEGATIVE = 1 << 3;
        }
    }

    impl Pattern {
        pub fn new(pattern: impl Into<BString>, mode: Mode) -> Self {
            Pattern {
                _inner: pattern.into(),
                _mode: mode,
            }
        }

        pub fn from_bytes(pattern: &BStr) -> Option<Self> {
            crate::parse(pattern).map(|(pattern, mode)| Self::new(pattern, mode))
        }

        pub fn matches(&self, _value: &BStr) -> bool {
            todo!()
        }
    }
}

mod parse;
pub use parse::pattern as parse;

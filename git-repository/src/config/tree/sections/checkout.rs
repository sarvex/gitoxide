use crate::config;
use crate::config::tree::{keys, Checkout, Key, Section};

impl Checkout {
    /// The `checkout.workers` key.
    pub const WORKERS: Workers = Workers::new_with_validate("workers", &config::Tree::CHECKOUT, validate::Workers);
}

/// The `checkout.workers` key.
pub type Workers = keys::Any<validate::Workers>;

impl Section for Checkout {
    fn name(&self) -> &str {
        "checkout"
    }

    fn keys(&self) -> &[&dyn Key] {
        &[&Self::WORKERS]
    }
}

mod workers {
    use crate::bstr::BStr;
    use crate::config::tree::checkout::Workers;
    use std::borrow::Cow;

    impl Workers {
        /// Return the amount of threads to use for checkout, with `0` meaning all available ones.
        pub fn try_into_thread_count(_value: Cow<'_, BStr>) -> Result<(), git_validate::reference::name::Error> {
            todo!()
        }
    }
}

///
pub mod validate {
    use crate::bstr::BStr;
    use crate::config::tree::keys;

    pub struct Workers;
    impl keys::Validate for Workers {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            super::Workers::try_into_thread_count(value.into())?;
            Ok(())
        }
    }
}

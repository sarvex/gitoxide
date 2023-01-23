use crate::bstr::BStr;
use crate::config::tree::{Key, Link, Section, SubSectionRequirement};
use std::error::Error;
use std::fmt::{Debug, Formatter};

/// Implements a value without any constraints, i.e. a any value.
pub struct Any<T: Validate = validate::All> {
    /// The key of the value in the git configuration.
    pub name: &'static str,
    /// The parent section of the key.
    pub section: &'static dyn Section,
    /// The subsection requirement to use.
    pub subsection_requirement: Option<SubSectionRequirement>,
    /// A link to other resources that might be eligible as value.
    pub link: Option<Link>,
    /// The way validation and transformation should happen.
    validate: T,
}

/// Init
impl Any<validate::All> {
    /// Create a new instance from `name` and `section`
    pub const fn new(name: &'static str, section: &'static dyn Section) -> Self {
        Any::new_with_validate(name, section, validate::All)
    }
}

/// Init other validate implementations
impl<T: Validate> Any<T> {
    /// Create a new instance from `name` and `section`
    pub const fn new_with_validate(name: &'static str, section: &'static dyn Section, validate: T) -> Self {
        Any {
            name,
            section,
            subsection_requirement: Some(SubSectionRequirement::Never),
            link: None,
            validate,
        }
    }
}

/// Builder
impl<T: Validate> Any<T> {
    /// Set the subsection requirement to non-default values.
    pub const fn with_subsection_requirement(mut self, requirement: Option<SubSectionRequirement>) -> Self {
        self.subsection_requirement = requirement;
        self
    }

    /// Associate an environment variable with this key.
    ///
    /// This is mainly useful for enriching error messages.
    pub const fn with_environment_override(mut self, var: &'static str) -> Self {
        self.link = Some(Link::EnvironmentOverride(var));
        self
    }

    /// Set a link to another key which serves as fallback to provide a value if this key is not set.
    pub const fn with_fallback(mut self, key: &'static dyn Key) -> Self {
        self.link = Some(Link::FallbackKey(key));
        self
    }
}

impl<T: Validate> Debug for Any<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.logical_name().fmt(f)
    }
}

impl<T: Validate> Key for Any<T> {
    fn name(&self) -> &str {
        self.name
    }

    fn validate(&self, value: &BStr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.validate.validate(value)
    }

    fn section(&self) -> &dyn Section {
        self.section
    }

    fn subsection_requirement(&self) -> Option<&SubSectionRequirement> {
        self.subsection_requirement.as_ref()
    }

    fn link(&self) -> Option<&Link> {
        self.link.as_ref()
    }
}

/// A key which represents a date.
pub type Time = Any<validate::Time>;

/// A key that represents a remote name.
pub type RemoteName = Any<validate::RemoteName>;

mod time {
    use crate::bstr::{BStr, ByteSlice};
    use crate::config::tree::keys::{validate, Time};
    use crate::config::tree::Section;
    use std::borrow::Cow;

    impl Time {
        /// Create a new instance.
        pub const fn new_time(name: &'static str, section: &'static dyn Section) -> Self {
            Self::new_with_validate(name, section, validate::Time)
        }

        /// Convert the `value` into a date if possible, with `now` as reference time for relative dates.
        pub fn try_into_time(
            &self,
            value: Cow<'_, BStr>,
            now: Option<std::time::SystemTime>,
        ) -> Result<git_date::Time, git_date::parse::Error> {
            git_date::parse(
                value
                    .as_ref()
                    .to_str()
                    .map_err(|_| git_date::parse::Error::InvalidDateString {
                        input: value.to_string(),
                    })?,
                now,
            )
        }
    }
}

impl RemoteName {
    /// Create a new instance.
    pub const fn new_remote_name(name: &'static str, section: &'static dyn Section) -> Self {
        Self::new_with_validate(name, section, validate::RemoteName)
    }
}

/// Provide a way to validate a value, or decode a value from `git-config`.
pub trait Validate {
    /// Validate `value` or return an error.
    fn validate(&self, value: &BStr) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

/// various implementations of the `Validate` trait.
pub mod validate {
    use crate::bstr::{BStr, ByteSlice};
    use crate::config::tree::keys::Validate;
    use crate::remote;
    use std::borrow::Cow;
    use std::error::Error;

    /// Everything is valid.
    #[derive(Default)]
    pub struct All;

    impl Validate for All {
        fn validate(&self, _value: &BStr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            Ok(())
        }
    }

    /// Values that parse as git dates are valid.
    #[derive(Default)]
    pub struct Time;

    impl Validate for Time {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
            git_date::parse(value.to_str()?, std::time::SystemTime::now().into())?;
            Ok(())
        }
    }

    /// Values that are git remotes, symbolic or urls
    #[derive(Default)]
    pub struct RemoteName;
    impl Validate for RemoteName {
        fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
            remote::Name::try_from(Cow::Borrowed(value))
                .map_err(|_| format!("Illformed UTF-8 in remote name: \"{}\"", value.to_str_lossy()))?;
            Ok(())
        }
    }
}

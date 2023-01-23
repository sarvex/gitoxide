use crate::bstr::{BStr, BString, ByteVec};
use std::borrow::Cow;

/// Provide information about a configuration section.
pub trait Section {
    /// The section name, like `remote` in `remote.origin.url`.
    fn name(&self) -> &str;
    /// The keys directly underneath it for carrying configuration values.
    fn keys(&self) -> &[&dyn Key];
    /// The list of sub-section names, which may be empty if there are no statically known sub-sections.
    fn sub_sections(&self) -> &[&dyn Section] {
        &[]
    }
    /// The parent section if this is a statically known sub-section.
    fn parent(&self) -> Option<&dyn Section> {
        None
    }
}

/// Determine how subsections may be used with a given key.
#[derive(Debug, Copy, Clone)]
pub enum SubSectionRequirement {
    /// Subsections must not be used, this key can only be below a section.
    Never,
    /// This key must always be a in a subsection, and `parameter` is the kind of
    /// value the sub-section represents if it is non-static.
    Always {
        /// The name of the parameter to show if the sub-section isn't static, like `section.<parameter>.key`.
        parameter: Option<&'static str>,
    },
}

/// A way to link a key with other resources.
#[derive(Debug, Copy, Clone)]
pub enum Link {
    /// The environment variable of the given name will override the value of this key.
    EnvironmentOverride(&'static str),
    /// This config key is used as fallback if this key isn't set.
    FallbackKey(&'static dyn Key),
}

/// A leaf-level entry in the git configuration, like `url` in `remote.origin.url`.
pub trait Key: std::fmt::Debug {
    /// The key's name, like `url` in `remote.origin.url`.
    fn name(&self) -> &str;
    /// See if `value` is allowed as value of this key, or return a descriptive error if it is not.
    fn validate(&self, value: &BStr) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
    /// foo
    fn validate_any<'a>(&self, value: Cow<'a, BStr>) -> Cow<'a, BStr> {
        value
    }
    /// The section containing this key. Git configuration has no free-standing keys, they are always underneath a section.
    fn section(&self) -> &dyn Section;
    /// The return value encodes three possible states to indicate subsection requirements
    /// * `None` = subsections may or may not be used, the most flexible setting.
    /// * `Some([Requirement][SubSectionRequirement])` = subsections must or must not be used, depending on the value
    fn subsection_requirement(&self) -> Option<&SubSectionRequirement> {
        Some(&SubSectionRequirement::Never)
    }
    /// Return the link to other resources, if available.
    fn link(&self) -> Option<&Link> {
        None
    }

    /// Return the name of an environment variable that would override this value (after following links until one is found).
    fn environment_override(&self) -> Option<&str> {
        let mut cursor = self.link()?;
        loop {
            match cursor {
                Link::EnvironmentOverride(name) => return Some(name),
                Link::FallbackKey(next) => {
                    cursor = next.link()?;
                }
            }
        }
    }
    /// Produce a name that describes how the name is composed. This is `core.bare` for statically known keys, or `branch.<name>.key`
    /// for complex ones.
    fn logical_name(&self) -> String {
        let section = self.section();
        let mut buf = String::new();
        let parameter = if let Some(parent) = section.parent() {
            buf.push_str(parent.name());
            buf.push('.');
            None
        } else {
            self.subsection_requirement().and_then(|requirement| match requirement {
                SubSectionRequirement::Always { parameter } => *parameter,
                SubSectionRequirement::Never => None,
            })
        };
        buf.push_str(section.name());
        buf.push('.');
        if let Some(parameter) = parameter {
            buf.push('<');
            buf.push_str(parameter);
            buf.push('>');
            buf.push('.');
        }
        buf.push_str(self.name());
        buf
    }

    /// The full name of the key for use in configuration overrides, like `core.bare`, or `remote.<subsection>.url` if `subsection` is
    /// not `None`.
    /// May fail if this key needs a subsection, or may not have a subsection.
    fn full_name(
        &self,
        subsection: Option<&BStr>,
    ) -> Result<BString, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let section = self.section();
        let mut buf = BString::default();
        let subsection = match self.subsection_requirement() {
            None => subsection,
            Some(requirement) => match (requirement, subsection) {
                (SubSectionRequirement::Never, Some(_)) => {
                    return Err(format!(
                        "The key named '{}' cannot be used with non-static subsections.",
                        self.logical_name()
                    )
                    .into());
                }
                (SubSectionRequirement::Always { parameter: _ }, None) => {
                    return Err(format!(
                        "The key named '{}' cannot be used without subsections.",
                        self.logical_name()
                    )
                    .into())
                }
                _ => subsection,
            },
        };

        if let Some(parent) = section.parent() {
            buf.push_str(parent.name());
            buf.push(b'.');
        }
        buf.push_str(section.name());
        buf.push(b'.');
        if let Some(subsection) = subsection {
            debug_assert!(
                section.parent().is_none(),
                "BUG: sections with parameterized sub-sections must be top-level sections"
            );
            buf.push_str(subsection);
            buf.push(b'.');
        }
        buf.push_str(self.name());
        Ok(buf)
    }

    /// Return an assignment with the keys full name to `value`, suitable for [configuration overrides][crate::open::Options::config_overrides()].
    /// Note that this will fail if the key requires a subsection name.
    fn validated_assignment(
        &self,
        value: &BStr,
    ) -> Result<BString, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.validate(value)?;
        let mut key = self.full_name(None)?;
        key.push(b'=');
        key.push_str(value);
        Ok(key)
    }

    /// Return an assignment to `value` with the keys full name within `subsection`, suitable for [configuration overrides][crate::open::Options::config_overrides()].
    /// Note that this is only valid if this key supports parameterized sub-sections, or else an error is returned.
    fn validated_assignment_with_subsection(
        &self,
        value: &BStr,
        subsection: &BStr,
    ) -> Result<BString, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.validate(value)?;
        let mut key = self.full_name(Some(subsection))?;
        key.push(b'=');
        key.push_str(value);
        Ok(key)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientResponseName {
    #[serde(default)]
    pub first: String,
    #[serde(default)]
    pub last: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred: Option<String>,
}

impl UpdatePatientResponseName {
    pub fn builder() -> UpdatePatientResponseNameBuilder {
        <UpdatePatientResponseNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseNameBuilder {
    first: Option<String>,
    last: Option<String>,
    middle: Option<String>,
    preferred: Option<String>,
}

impl UpdatePatientResponseNameBuilder {
    pub fn first(mut self, value: impl Into<String>) -> Self {
        self.first = Some(value.into());
        self
    }

    pub fn last(mut self, value: impl Into<String>) -> Self {
        self.last = Some(value.into());
        self
    }

    pub fn middle(mut self, value: impl Into<String>) -> Self {
        self.middle = Some(value.into());
        self
    }

    pub fn preferred(mut self, value: impl Into<String>) -> Self {
        self.preferred = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponseName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first`](UpdatePatientResponseNameBuilder::first)
    /// - [`last`](UpdatePatientResponseNameBuilder::last)
    pub fn build(self) -> Result<UpdatePatientResponseName, BuildError> {
        Ok(UpdatePatientResponseName {
            first: self
                .first
                .ok_or_else(|| BuildError::missing_field("first"))?,
            last: self.last.ok_or_else(|| BuildError::missing_field("last"))?,
            middle: self.middle,
            preferred: self.preferred,
        })
    }
}

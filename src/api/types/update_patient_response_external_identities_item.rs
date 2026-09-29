pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientResponseExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl UpdatePatientResponseExternalIdentitiesItem {
    pub fn builder() -> UpdatePatientResponseExternalIdentitiesItemBuilder {
        <UpdatePatientResponseExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl UpdatePatientResponseExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponseExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](UpdatePatientResponseExternalIdentitiesItemBuilder::source)
    /// - [`value`](UpdatePatientResponseExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<UpdatePatientResponseExternalIdentitiesItem, BuildError> {
        Ok(UpdatePatientResponseExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

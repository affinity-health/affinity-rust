pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientRequestExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl UpdatePatientRequestExternalIdentitiesItem {
    pub fn builder() -> UpdatePatientRequestExternalIdentitiesItemBuilder {
        <UpdatePatientRequestExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl UpdatePatientRequestExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientRequestExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](UpdatePatientRequestExternalIdentitiesItemBuilder::source)
    /// - [`value`](UpdatePatientRequestExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<UpdatePatientRequestExternalIdentitiesItem, BuildError> {
        Ok(UpdatePatientRequestExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

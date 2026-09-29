pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePatientRequestExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl CreatePatientRequestExternalIdentitiesItem {
    pub fn builder() -> CreatePatientRequestExternalIdentitiesItemBuilder {
        <CreatePatientRequestExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientRequestExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl CreatePatientRequestExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientRequestExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](CreatePatientRequestExternalIdentitiesItemBuilder::source)
    /// - [`value`](CreatePatientRequestExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<CreatePatientRequestExternalIdentitiesItem, BuildError> {
        Ok(CreatePatientRequestExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

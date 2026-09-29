pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePatientResponseExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl CreatePatientResponseExternalIdentitiesItem {
    pub fn builder() -> CreatePatientResponseExternalIdentitiesItemBuilder {
        <CreatePatientResponseExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientResponseExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl CreatePatientResponseExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientResponseExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](CreatePatientResponseExternalIdentitiesItemBuilder::source)
    /// - [`value`](CreatePatientResponseExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<CreatePatientResponseExternalIdentitiesItem, BuildError> {
        Ok(CreatePatientResponseExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

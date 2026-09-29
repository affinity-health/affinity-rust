pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPatientResponseExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl GetPatientResponseExternalIdentitiesItem {
    pub fn builder() -> GetPatientResponseExternalIdentitiesItemBuilder {
        <GetPatientResponseExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPatientResponseExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl GetPatientResponseExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPatientResponseExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](GetPatientResponseExternalIdentitiesItemBuilder::source)
    /// - [`value`](GetPatientResponseExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<GetPatientResponseExternalIdentitiesItem, BuildError> {
        Ok(GetPatientResponseExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

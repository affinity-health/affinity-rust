pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPatientExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl PreviewOrderRequestPatientExternalIdentitiesItem {
    pub fn builder() -> PreviewOrderRequestPatientExternalIdentitiesItemBuilder {
        <PreviewOrderRequestPatientExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPatientExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl PreviewOrderRequestPatientExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPatientExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](PreviewOrderRequestPatientExternalIdentitiesItemBuilder::source)
    /// - [`value`](PreviewOrderRequestPatientExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<PreviewOrderRequestPatientExternalIdentitiesItem, BuildError> {
        Ok(PreviewOrderRequestPatientExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseOrderInputPatientExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl PreviewOrderResponseOrderInputPatientExternalIdentitiesItem {
    pub fn builder() -> PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder {
        <PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPatientExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder::source)
    /// - [`value`](PreviewOrderResponseOrderInputPatientExternalIdentitiesItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPatientExternalIdentitiesItem, BuildError> {
        Ok(
            PreviewOrderResponseOrderInputPatientExternalIdentitiesItem {
                source: self
                    .source
                    .ok_or_else(|| BuildError::missing_field("source"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}

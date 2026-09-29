pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPatientExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl CreateOrderRequestPatientExternalIdentitiesItem {
    pub fn builder() -> CreateOrderRequestPatientExternalIdentitiesItemBuilder {
        <CreateOrderRequestPatientExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPatientExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl CreateOrderRequestPatientExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPatientExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](CreateOrderRequestPatientExternalIdentitiesItemBuilder::source)
    /// - [`value`](CreateOrderRequestPatientExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<CreateOrderRequestPatientExternalIdentitiesItem, BuildError> {
        Ok(CreateOrderRequestPatientExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

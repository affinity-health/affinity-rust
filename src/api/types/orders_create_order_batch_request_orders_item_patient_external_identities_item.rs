pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItem {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder {
        <CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder::source)
    /// - [`value`](CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItem, BuildError> {
        Ok(
            CreateOrderBatchRequestOrdersItemPatientExternalIdentitiesItem {
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

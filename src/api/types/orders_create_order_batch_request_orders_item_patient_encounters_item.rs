pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestOrdersItemPatientEncountersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    pub occurred_at: String,
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    pub r#type: String,
}

impl CreateOrderBatchRequestOrdersItemPatientEncountersItem {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder {
        <CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder {
    notes: Option<String>,
    occurred_at: Option<String>,
    provider_name: Option<String>,
    r#type: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder {
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: impl Into<String>) -> Self {
        self.occurred_at = Some(value.into());
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPatientEncountersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`occurred_at`](CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder::occurred_at)
    /// - [`r#type`](CreateOrderBatchRequestOrdersItemPatientEncountersItemBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<CreateOrderBatchRequestOrdersItemPatientEncountersItem, BuildError> {
        Ok(CreateOrderBatchRequestOrdersItemPatientEncountersItem {
            notes: self.notes,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            provider_name: self.provider_name,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseCancellationOutcomesItem {
    #[serde(rename = "cancellationId")]
    #[serde(default)]
    pub cancellation_id: String,
    #[serde(rename = "fulfillmentId")]
    #[serde(default)]
    pub fulfillment_id: String,
    pub status: CancelOrderResponseCancellationOutcomesItemStatus,
}

impl CancelOrderResponseCancellationOutcomesItem {
    pub fn builder() -> CancelOrderResponseCancellationOutcomesItemBuilder {
        <CancelOrderResponseCancellationOutcomesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseCancellationOutcomesItemBuilder {
    cancellation_id: Option<String>,
    fulfillment_id: Option<String>,
    status: Option<CancelOrderResponseCancellationOutcomesItemStatus>,
}

impl CancelOrderResponseCancellationOutcomesItemBuilder {
    pub fn cancellation_id(mut self, value: impl Into<String>) -> Self {
        self.cancellation_id = Some(value.into());
        self
    }

    pub fn fulfillment_id(mut self, value: impl Into<String>) -> Self {
        self.fulfillment_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CancelOrderResponseCancellationOutcomesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseCancellationOutcomesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cancellation_id`](CancelOrderResponseCancellationOutcomesItemBuilder::cancellation_id)
    /// - [`fulfillment_id`](CancelOrderResponseCancellationOutcomesItemBuilder::fulfillment_id)
    /// - [`status`](CancelOrderResponseCancellationOutcomesItemBuilder::status)
    pub fn build(self) -> Result<CancelOrderResponseCancellationOutcomesItem, BuildError> {
        Ok(CancelOrderResponseCancellationOutcomesItem {
            cancellation_id: self
                .cancellation_id
                .ok_or_else(|| BuildError::missing_field("cancellation_id"))?,
            fulfillment_id: self
                .fulfillment_id
                .ok_or_else(|| BuildError::missing_field("fulfillment_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

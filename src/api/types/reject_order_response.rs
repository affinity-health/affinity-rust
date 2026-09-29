pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RejectOrderResponse {
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "rejectedAt")]
    #[serde(default)]
    pub rejected_at: String,
    #[serde(default)]
    pub reason: String,
    pub status: RejectOrderResponseStatus,
}

impl RejectOrderResponse {
    pub fn builder() -> RejectOrderResponseBuilder {
        <RejectOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectOrderResponseBuilder {
    order_id: Option<String>,
    rejected_at: Option<String>,
    reason: Option<String>,
    status: Option<RejectOrderResponseStatus>,
}

impl RejectOrderResponseBuilder {
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn rejected_at(mut self, value: impl Into<String>) -> Self {
        self.rejected_at = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: RejectOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RejectOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](RejectOrderResponseBuilder::order_id)
    /// - [`rejected_at`](RejectOrderResponseBuilder::rejected_at)
    /// - [`reason`](RejectOrderResponseBuilder::reason)
    /// - [`status`](RejectOrderResponseBuilder::status)
    pub fn build(self) -> Result<RejectOrderResponse, BuildError> {
        Ok(RejectOrderResponse {
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            rejected_at: self
                .rejected_at
                .ok_or_else(|| BuildError::missing_field("rejected_at"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

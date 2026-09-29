pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayWebhookEventResponseDeliveriesItem {
    #[serde(rename = "automaticAttemptCount")]
    pub automatic_attempt_count: ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount,
    #[serde(rename = "endpointId")]
    #[serde(default)]
    pub endpoint_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(rename = "nextAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_attempt_at: Option<String>,
    pub status: ReplayWebhookEventResponseDeliveriesItemStatus,
}

impl ReplayWebhookEventResponseDeliveriesItem {
    pub fn builder() -> ReplayWebhookEventResponseDeliveriesItemBuilder {
        <ReplayWebhookEventResponseDeliveriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayWebhookEventResponseDeliveriesItemBuilder {
    automatic_attempt_count: Option<ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount>,
    endpoint_id: Option<String>,
    id: Option<String>,
    last_error_code: Option<String>,
    last_error_message: Option<String>,
    next_attempt_at: Option<String>,
    status: Option<ReplayWebhookEventResponseDeliveriesItemStatus>,
}

impl ReplayWebhookEventResponseDeliveriesItemBuilder {
    pub fn automatic_attempt_count(
        mut self,
        value: ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount,
    ) -> Self {
        self.automatic_attempt_count = Some(value);
        self
    }

    pub fn endpoint_id(mut self, value: impl Into<String>) -> Self {
        self.endpoint_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_error_code(mut self, value: impl Into<String>) -> Self {
        self.last_error_code = Some(value.into());
        self
    }

    pub fn last_error_message(mut self, value: impl Into<String>) -> Self {
        self.last_error_message = Some(value.into());
        self
    }

    pub fn next_attempt_at(mut self, value: impl Into<String>) -> Self {
        self.next_attempt_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: ReplayWebhookEventResponseDeliveriesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayWebhookEventResponseDeliveriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automatic_attempt_count`](ReplayWebhookEventResponseDeliveriesItemBuilder::automatic_attempt_count)
    /// - [`endpoint_id`](ReplayWebhookEventResponseDeliveriesItemBuilder::endpoint_id)
    /// - [`id`](ReplayWebhookEventResponseDeliveriesItemBuilder::id)
    /// - [`status`](ReplayWebhookEventResponseDeliveriesItemBuilder::status)
    pub fn build(self) -> Result<ReplayWebhookEventResponseDeliveriesItem, BuildError> {
        Ok(ReplayWebhookEventResponseDeliveriesItem {
            automatic_attempt_count: self
                .automatic_attempt_count
                .ok_or_else(|| BuildError::missing_field("automatic_attempt_count"))?,
            endpoint_id: self
                .endpoint_id
                .ok_or_else(|| BuildError::missing_field("endpoint_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_error_code: self.last_error_code,
            last_error_message: self.last_error_message,
            next_attempt_at: self.next_attempt_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

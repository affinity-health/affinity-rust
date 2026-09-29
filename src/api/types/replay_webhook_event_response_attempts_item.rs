pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayWebhookEventResponseAttemptsItem {
    #[serde(rename = "deliveryId")]
    #[serde(default)]
    pub delivery_id: String,
    #[serde(rename = "endpointId")]
    #[serde(default)]
    pub endpoint_id: String,
    #[serde(rename = "attemptNumber")]
    #[serde(default)]
    pub attempt_number: i64,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "durationMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<ReplayWebhookEventResponseAttemptsItemDurationMs>,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<ReplayWebhookEventResponseAttemptsItemResponseStatus>,
    pub trigger: ReplayWebhookEventResponseAttemptsItemTrigger,
}

impl ReplayWebhookEventResponseAttemptsItem {
    pub fn builder() -> ReplayWebhookEventResponseAttemptsItemBuilder {
        <ReplayWebhookEventResponseAttemptsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayWebhookEventResponseAttemptsItemBuilder {
    delivery_id: Option<String>,
    endpoint_id: Option<String>,
    attempt_number: Option<i64>,
    completed_at: Option<String>,
    duration_ms: Option<ReplayWebhookEventResponseAttemptsItemDurationMs>,
    error_code: Option<String>,
    error_message: Option<String>,
    id: Option<String>,
    requested_at: Option<String>,
    response_status: Option<ReplayWebhookEventResponseAttemptsItemResponseStatus>,
    trigger: Option<ReplayWebhookEventResponseAttemptsItemTrigger>,
}

impl ReplayWebhookEventResponseAttemptsItemBuilder {
    pub fn delivery_id(mut self, value: impl Into<String>) -> Self {
        self.delivery_id = Some(value.into());
        self
    }

    pub fn endpoint_id(mut self, value: impl Into<String>) -> Self {
        self.endpoint_id = Some(value.into());
        self
    }

    pub fn attempt_number(mut self, value: i64) -> Self {
        self.attempt_number = Some(value);
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: ReplayWebhookEventResponseAttemptsItemDurationMs) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn error_code(mut self, value: impl Into<String>) -> Self {
        self.error_code = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn requested_at(mut self, value: impl Into<String>) -> Self {
        self.requested_at = Some(value.into());
        self
    }

    pub fn response_status(
        mut self,
        value: ReplayWebhookEventResponseAttemptsItemResponseStatus,
    ) -> Self {
        self.response_status = Some(value);
        self
    }

    pub fn trigger(mut self, value: ReplayWebhookEventResponseAttemptsItemTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayWebhookEventResponseAttemptsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_id`](ReplayWebhookEventResponseAttemptsItemBuilder::delivery_id)
    /// - [`endpoint_id`](ReplayWebhookEventResponseAttemptsItemBuilder::endpoint_id)
    /// - [`attempt_number`](ReplayWebhookEventResponseAttemptsItemBuilder::attempt_number)
    /// - [`id`](ReplayWebhookEventResponseAttemptsItemBuilder::id)
    /// - [`requested_at`](ReplayWebhookEventResponseAttemptsItemBuilder::requested_at)
    /// - [`trigger`](ReplayWebhookEventResponseAttemptsItemBuilder::trigger)
    pub fn build(self) -> Result<ReplayWebhookEventResponseAttemptsItem, BuildError> {
        Ok(ReplayWebhookEventResponseAttemptsItem {
            delivery_id: self
                .delivery_id
                .ok_or_else(|| BuildError::missing_field("delivery_id"))?,
            endpoint_id: self
                .endpoint_id
                .ok_or_else(|| BuildError::missing_field("endpoint_id"))?,
            attempt_number: self
                .attempt_number
                .ok_or_else(|| BuildError::missing_field("attempt_number"))?,
            completed_at: self.completed_at,
            duration_ms: self.duration_ms,
            error_code: self.error_code,
            error_message: self.error_message,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            requested_at: self
                .requested_at
                .ok_or_else(|| BuildError::missing_field("requested_at"))?,
            response_status: self.response_status,
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
        })
    }
}

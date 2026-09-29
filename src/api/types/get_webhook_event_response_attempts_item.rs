pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetWebhookEventResponseAttemptsItem {
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
    pub duration_ms: Option<GetWebhookEventResponseAttemptsItemDurationMs>,
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
    pub response_status: Option<GetWebhookEventResponseAttemptsItemResponseStatus>,
    pub trigger: GetWebhookEventResponseAttemptsItemTrigger,
}

impl GetWebhookEventResponseAttemptsItem {
    pub fn builder() -> GetWebhookEventResponseAttemptsItemBuilder {
        <GetWebhookEventResponseAttemptsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebhookEventResponseAttemptsItemBuilder {
    delivery_id: Option<String>,
    endpoint_id: Option<String>,
    attempt_number: Option<i64>,
    completed_at: Option<String>,
    duration_ms: Option<GetWebhookEventResponseAttemptsItemDurationMs>,
    error_code: Option<String>,
    error_message: Option<String>,
    id: Option<String>,
    requested_at: Option<String>,
    response_status: Option<GetWebhookEventResponseAttemptsItemResponseStatus>,
    trigger: Option<GetWebhookEventResponseAttemptsItemTrigger>,
}

impl GetWebhookEventResponseAttemptsItemBuilder {
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

    pub fn duration_ms(mut self, value: GetWebhookEventResponseAttemptsItemDurationMs) -> Self {
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
        value: GetWebhookEventResponseAttemptsItemResponseStatus,
    ) -> Self {
        self.response_status = Some(value);
        self
    }

    pub fn trigger(mut self, value: GetWebhookEventResponseAttemptsItemTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWebhookEventResponseAttemptsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_id`](GetWebhookEventResponseAttemptsItemBuilder::delivery_id)
    /// - [`endpoint_id`](GetWebhookEventResponseAttemptsItemBuilder::endpoint_id)
    /// - [`attempt_number`](GetWebhookEventResponseAttemptsItemBuilder::attempt_number)
    /// - [`id`](GetWebhookEventResponseAttemptsItemBuilder::id)
    /// - [`requested_at`](GetWebhookEventResponseAttemptsItemBuilder::requested_at)
    /// - [`trigger`](GetWebhookEventResponseAttemptsItemBuilder::trigger)
    pub fn build(self) -> Result<GetWebhookEventResponseAttemptsItem, BuildError> {
        Ok(GetWebhookEventResponseAttemptsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayWebhookEventResponse {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    pub api_version: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub object: ReplayWebhookEventResponseObject,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    pub status: ReplayWebhookEventResponseStatus,
    #[serde(default)]
    pub attempts: Vec<ReplayWebhookEventResponseAttemptsItem>,
    #[serde(default)]
    pub deliveries: Vec<ReplayWebhookEventResponseDeliveriesItem>,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
}

impl ReplayWebhookEventResponse {
    pub fn builder() -> ReplayWebhookEventResponseBuilder {
        <ReplayWebhookEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayWebhookEventResponseBuilder {
    api_version: Option<String>,
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<ReplayWebhookEventResponseObject>,
    resource_id: Option<String>,
    resource_type: Option<String>,
    status: Option<ReplayWebhookEventResponseStatus>,
    attempts: Option<Vec<ReplayWebhookEventResponseAttemptsItem>>,
    deliveries: Option<Vec<ReplayWebhookEventResponseDeliveriesItem>>,
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl ReplayWebhookEventResponseBuilder {
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.api_version = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: ReplayWebhookEventResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: impl Into<String>) -> Self {
        self.resource_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: ReplayWebhookEventResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn attempts(mut self, value: Vec<ReplayWebhookEventResponseAttemptsItem>) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn deliveries(mut self, value: Vec<ReplayWebhookEventResponseDeliveriesItem>) -> Self {
        self.deliveries = Some(value);
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayWebhookEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](ReplayWebhookEventResponseBuilder::api_version)
    /// - [`created_at`](ReplayWebhookEventResponseBuilder::created_at)
    /// - [`event_type`](ReplayWebhookEventResponseBuilder::event_type)
    /// - [`id`](ReplayWebhookEventResponseBuilder::id)
    /// - [`livemode`](ReplayWebhookEventResponseBuilder::livemode)
    /// - [`object`](ReplayWebhookEventResponseBuilder::object)
    /// - [`resource_id`](ReplayWebhookEventResponseBuilder::resource_id)
    /// - [`resource_type`](ReplayWebhookEventResponseBuilder::resource_type)
    /// - [`status`](ReplayWebhookEventResponseBuilder::status)
    /// - [`attempts`](ReplayWebhookEventResponseBuilder::attempts)
    /// - [`deliveries`](ReplayWebhookEventResponseBuilder::deliveries)
    /// - [`payload`](ReplayWebhookEventResponseBuilder::payload)
    pub fn build(self) -> Result<ReplayWebhookEventResponse, BuildError> {
        Ok(ReplayWebhookEventResponse {
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            attempts: self
                .attempts
                .ok_or_else(|| BuildError::missing_field("attempts"))?,
            deliveries: self
                .deliveries
                .ok_or_else(|| BuildError::missing_field("deliveries"))?,
            payload: self
                .payload
                .ok_or_else(|| BuildError::missing_field("payload"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetWebhookEventResponse {
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
    pub object: GetWebhookEventResponseObject,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    pub status: GetWebhookEventResponseStatus,
    #[serde(default)]
    pub attempts: Vec<GetWebhookEventResponseAttemptsItem>,
    #[serde(default)]
    pub deliveries: Vec<GetWebhookEventResponseDeliveriesItem>,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
}

impl GetWebhookEventResponse {
    pub fn builder() -> GetWebhookEventResponseBuilder {
        <GetWebhookEventResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebhookEventResponseBuilder {
    api_version: Option<String>,
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<GetWebhookEventResponseObject>,
    resource_id: Option<String>,
    resource_type: Option<String>,
    status: Option<GetWebhookEventResponseStatus>,
    attempts: Option<Vec<GetWebhookEventResponseAttemptsItem>>,
    deliveries: Option<Vec<GetWebhookEventResponseDeliveriesItem>>,
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl GetWebhookEventResponseBuilder {
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

    pub fn object(mut self, value: GetWebhookEventResponseObject) -> Self {
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

    pub fn status(mut self, value: GetWebhookEventResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn attempts(mut self, value: Vec<GetWebhookEventResponseAttemptsItem>) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn deliveries(mut self, value: Vec<GetWebhookEventResponseDeliveriesItem>) -> Self {
        self.deliveries = Some(value);
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetWebhookEventResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](GetWebhookEventResponseBuilder::api_version)
    /// - [`created_at`](GetWebhookEventResponseBuilder::created_at)
    /// - [`event_type`](GetWebhookEventResponseBuilder::event_type)
    /// - [`id`](GetWebhookEventResponseBuilder::id)
    /// - [`livemode`](GetWebhookEventResponseBuilder::livemode)
    /// - [`object`](GetWebhookEventResponseBuilder::object)
    /// - [`resource_id`](GetWebhookEventResponseBuilder::resource_id)
    /// - [`resource_type`](GetWebhookEventResponseBuilder::resource_type)
    /// - [`status`](GetWebhookEventResponseBuilder::status)
    /// - [`attempts`](GetWebhookEventResponseBuilder::attempts)
    /// - [`deliveries`](GetWebhookEventResponseBuilder::deliveries)
    /// - [`payload`](GetWebhookEventResponseBuilder::payload)
    pub fn build(self) -> Result<GetWebhookEventResponse, BuildError> {
        Ok(GetWebhookEventResponse {
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

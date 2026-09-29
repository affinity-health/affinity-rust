pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookEndpointsResponseDataItem {
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "practiceIds")]
    #[serde(default)]
    pub practice_ids: Vec<String>,
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    pub api_version: String,
    #[serde(rename = "consecutiveFailures")]
    #[serde(default)]
    pub consecutive_failures: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub object: ListWebhookEndpointsResponseDataItemObject,
    #[serde(rename = "payloadStyle")]
    pub payload_style: ListWebhookEndpointsResponseDataItemPayloadStyle,
    pub status: ListWebhookEndpointsResponseDataItemStatus,
    #[serde(rename = "subscribedEvents")]
    #[serde(default)]
    pub subscribed_events: Vec<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub url: String,
}

impl ListWebhookEndpointsResponseDataItem {
    pub fn builder() -> ListWebhookEndpointsResponseDataItemBuilder {
        <ListWebhookEndpointsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEndpointsResponseDataItemBuilder {
    organization_id: Option<String>,
    practice_ids: Option<Vec<String>>,
    api_version: Option<String>,
    consecutive_failures: Option<i64>,
    created_at: Option<String>,
    description: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<ListWebhookEndpointsResponseDataItemObject>,
    payload_style: Option<ListWebhookEndpointsResponseDataItemPayloadStyle>,
    status: Option<ListWebhookEndpointsResponseDataItemStatus>,
    subscribed_events: Option<Vec<String>>,
    updated_at: Option<String>,
    url: Option<String>,
}

impl ListWebhookEndpointsResponseDataItemBuilder {
    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn practice_ids(mut self, value: Vec<String>) -> Self {
        self.practice_ids = Some(value);
        self
    }

    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.api_version = Some(value.into());
        self
    }

    pub fn consecutive_failures(mut self, value: i64) -> Self {
        self.consecutive_failures = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn object(mut self, value: ListWebhookEndpointsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payload_style(
        mut self,
        value: ListWebhookEndpointsResponseDataItemPayloadStyle,
    ) -> Self {
        self.payload_style = Some(value);
        self
    }

    pub fn status(mut self, value: ListWebhookEndpointsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscribed_events(mut self, value: Vec<String>) -> Self {
        self.subscribed_events = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEndpointsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_id`](ListWebhookEndpointsResponseDataItemBuilder::organization_id)
    /// - [`practice_ids`](ListWebhookEndpointsResponseDataItemBuilder::practice_ids)
    /// - [`api_version`](ListWebhookEndpointsResponseDataItemBuilder::api_version)
    /// - [`consecutive_failures`](ListWebhookEndpointsResponseDataItemBuilder::consecutive_failures)
    /// - [`created_at`](ListWebhookEndpointsResponseDataItemBuilder::created_at)
    /// - [`description`](ListWebhookEndpointsResponseDataItemBuilder::description)
    /// - [`id`](ListWebhookEndpointsResponseDataItemBuilder::id)
    /// - [`livemode`](ListWebhookEndpointsResponseDataItemBuilder::livemode)
    /// - [`object`](ListWebhookEndpointsResponseDataItemBuilder::object)
    /// - [`payload_style`](ListWebhookEndpointsResponseDataItemBuilder::payload_style)
    /// - [`status`](ListWebhookEndpointsResponseDataItemBuilder::status)
    /// - [`subscribed_events`](ListWebhookEndpointsResponseDataItemBuilder::subscribed_events)
    /// - [`updated_at`](ListWebhookEndpointsResponseDataItemBuilder::updated_at)
    /// - [`url`](ListWebhookEndpointsResponseDataItemBuilder::url)
    pub fn build(self) -> Result<ListWebhookEndpointsResponseDataItem, BuildError> {
        Ok(ListWebhookEndpointsResponseDataItem {
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            practice_ids: self
                .practice_ids
                .ok_or_else(|| BuildError::missing_field("practice_ids"))?,
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            consecutive_failures: self
                .consecutive_failures
                .ok_or_else(|| BuildError::missing_field("consecutive_failures"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payload_style: self
                .payload_style
                .ok_or_else(|| BuildError::missing_field("payload_style"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subscribed_events: self
                .subscribed_events
                .ok_or_else(|| BuildError::missing_field("subscribed_events"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

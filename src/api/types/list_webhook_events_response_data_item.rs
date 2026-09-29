pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookEventsResponseDataItem {
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
    pub object: ListWebhookEventsResponseDataItemObject,
    #[serde(rename = "resourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
    pub status: ListWebhookEventsResponseDataItemStatus,
}

impl ListWebhookEventsResponseDataItem {
    pub fn builder() -> ListWebhookEventsResponseDataItemBuilder {
        <ListWebhookEventsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEventsResponseDataItemBuilder {
    api_version: Option<String>,
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<ListWebhookEventsResponseDataItemObject>,
    resource_id: Option<String>,
    resource_type: Option<String>,
    status: Option<ListWebhookEventsResponseDataItemStatus>,
}

impl ListWebhookEventsResponseDataItemBuilder {
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

    pub fn object(mut self, value: ListWebhookEventsResponseDataItemObject) -> Self {
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

    pub fn status(mut self, value: ListWebhookEventsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEventsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](ListWebhookEventsResponseDataItemBuilder::api_version)
    /// - [`created_at`](ListWebhookEventsResponseDataItemBuilder::created_at)
    /// - [`event_type`](ListWebhookEventsResponseDataItemBuilder::event_type)
    /// - [`id`](ListWebhookEventsResponseDataItemBuilder::id)
    /// - [`livemode`](ListWebhookEventsResponseDataItemBuilder::livemode)
    /// - [`object`](ListWebhookEventsResponseDataItemBuilder::object)
    /// - [`resource_id`](ListWebhookEventsResponseDataItemBuilder::resource_id)
    /// - [`resource_type`](ListWebhookEventsResponseDataItemBuilder::resource_type)
    /// - [`status`](ListWebhookEventsResponseDataItemBuilder::status)
    pub fn build(self) -> Result<ListWebhookEventsResponseDataItem, BuildError> {
        Ok(ListWebhookEventsResponseDataItem {
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
        })
    }
}

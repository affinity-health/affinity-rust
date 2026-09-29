pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrderEventsResponseDataItem {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    pub object: ListOrderEventsResponseDataItemObject,
}

impl ListOrderEventsResponseDataItem {
    pub fn builder() -> ListOrderEventsResponseDataItemBuilder {
        <ListOrderEventsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrderEventsResponseDataItemBuilder {
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    message: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    object: Option<ListOrderEventsResponseDataItemObject>,
}

impl ListOrderEventsResponseDataItemBuilder {
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

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn object(mut self, value: ListOrderEventsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrderEventsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListOrderEventsResponseDataItemBuilder::created_at)
    /// - [`event_type`](ListOrderEventsResponseDataItemBuilder::event_type)
    /// - [`id`](ListOrderEventsResponseDataItemBuilder::id)
    /// - [`message`](ListOrderEventsResponseDataItemBuilder::message)
    /// - [`metadata`](ListOrderEventsResponseDataItemBuilder::metadata)
    /// - [`object`](ListOrderEventsResponseDataItemBuilder::object)
    pub fn build(self) -> Result<ListOrderEventsResponseDataItem, BuildError> {
        Ok(ListOrderEventsResponseDataItem {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}

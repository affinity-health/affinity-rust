pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOrderResponseLifecycleEventsItem {
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
    pub source: GetOrderResponseLifecycleEventsItemSource,
}

impl GetOrderResponseLifecycleEventsItem {
    pub fn builder() -> GetOrderResponseLifecycleEventsItemBuilder {
        <GetOrderResponseLifecycleEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseLifecycleEventsItemBuilder {
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    message: Option<String>,
    source: Option<GetOrderResponseLifecycleEventsItemSource>,
}

impl GetOrderResponseLifecycleEventsItemBuilder {
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

    pub fn source(mut self, value: GetOrderResponseLifecycleEventsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseLifecycleEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GetOrderResponseLifecycleEventsItemBuilder::created_at)
    /// - [`event_type`](GetOrderResponseLifecycleEventsItemBuilder::event_type)
    /// - [`id`](GetOrderResponseLifecycleEventsItemBuilder::id)
    /// - [`message`](GetOrderResponseLifecycleEventsItemBuilder::message)
    /// - [`source`](GetOrderResponseLifecycleEventsItemBuilder::source)
    pub fn build(self) -> Result<GetOrderResponseLifecycleEventsItem, BuildError> {
        Ok(GetOrderResponseLifecycleEventsItem {
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
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseLifecycleEventsItem {
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
    pub source: CancelOrderResponseLifecycleEventsItemSource,
}

impl CancelOrderResponseLifecycleEventsItem {
    pub fn builder() -> CancelOrderResponseLifecycleEventsItemBuilder {
        <CancelOrderResponseLifecycleEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseLifecycleEventsItemBuilder {
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    message: Option<String>,
    source: Option<CancelOrderResponseLifecycleEventsItemSource>,
}

impl CancelOrderResponseLifecycleEventsItemBuilder {
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

    pub fn source(mut self, value: CancelOrderResponseLifecycleEventsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseLifecycleEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CancelOrderResponseLifecycleEventsItemBuilder::created_at)
    /// - [`event_type`](CancelOrderResponseLifecycleEventsItemBuilder::event_type)
    /// - [`id`](CancelOrderResponseLifecycleEventsItemBuilder::id)
    /// - [`message`](CancelOrderResponseLifecycleEventsItemBuilder::message)
    /// - [`source`](CancelOrderResponseLifecycleEventsItemBuilder::source)
    pub fn build(self) -> Result<CancelOrderResponseLifecycleEventsItem, BuildError> {
        Ok(CancelOrderResponseLifecycleEventsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TestWebhookEndpointResponse {
    #[serde(rename = "eventId")]
    #[serde(default)]
    pub event_id: String,
}

impl TestWebhookEndpointResponse {
    pub fn builder() -> TestWebhookEndpointResponseBuilder {
        <TestWebhookEndpointResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TestWebhookEndpointResponseBuilder {
    event_id: Option<String>,
}

impl TestWebhookEndpointResponseBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TestWebhookEndpointResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](TestWebhookEndpointResponseBuilder::event_id)
    pub fn build(self) -> Result<TestWebhookEndpointResponse, BuildError> {
        Ok(TestWebhookEndpointResponse {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookEndpointRequest {
    #[serde(rename = "practiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "payloadStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_style: Option<CreateWebhookEndpointRequestPayloadStyle>,
    #[serde(rename = "subscribedEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_events: Option<Vec<CreateWebhookEndpointRequestSubscribedEventsItem>>,
    #[serde(default)]
    pub url: String,
}

impl CreateWebhookEndpointRequest {
    pub fn builder() -> CreateWebhookEndpointRequestBuilder {
        <CreateWebhookEndpointRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookEndpointRequestBuilder {
    practice_ids: Option<Vec<String>>,
    description: Option<String>,
    payload_style: Option<CreateWebhookEndpointRequestPayloadStyle>,
    subscribed_events: Option<Vec<CreateWebhookEndpointRequestSubscribedEventsItem>>,
    url: Option<String>,
}

impl CreateWebhookEndpointRequestBuilder {
    pub fn practice_ids(mut self, value: Vec<String>) -> Self {
        self.practice_ids = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn payload_style(mut self, value: CreateWebhookEndpointRequestPayloadStyle) -> Self {
        self.payload_style = Some(value);
        self
    }

    pub fn subscribed_events(
        mut self,
        value: Vec<CreateWebhookEndpointRequestSubscribedEventsItem>,
    ) -> Self {
        self.subscribed_events = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookEndpointRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateWebhookEndpointRequestBuilder::url)
    pub fn build(self) -> Result<CreateWebhookEndpointRequest, BuildError> {
        Ok(CreateWebhookEndpointRequest {
            practice_ids: self.practice_ids,
            description: self.description,
            payload_style: self.payload_style,
            subscribed_events: self.subscribed_events,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

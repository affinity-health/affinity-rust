pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookEndpointRequest {
    #[serde(rename = "practiceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "payloadStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_style: Option<UpdateWebhookEndpointRequestPayloadStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateWebhookEndpointRequestStatus>,
    #[serde(rename = "subscribedEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_events: Option<Vec<UpdateWebhookEndpointRequestSubscribedEventsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateWebhookEndpointRequest {
    pub fn builder() -> UpdateWebhookEndpointRequestBuilder {
        <UpdateWebhookEndpointRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookEndpointRequestBuilder {
    practice_ids: Option<Vec<String>>,
    description: Option<String>,
    payload_style: Option<UpdateWebhookEndpointRequestPayloadStyle>,
    status: Option<UpdateWebhookEndpointRequestStatus>,
    subscribed_events: Option<Vec<UpdateWebhookEndpointRequestSubscribedEventsItem>>,
    url: Option<String>,
}

impl UpdateWebhookEndpointRequestBuilder {
    pub fn practice_ids(mut self, value: Vec<String>) -> Self {
        self.practice_ids = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn payload_style(mut self, value: UpdateWebhookEndpointRequestPayloadStyle) -> Self {
        self.payload_style = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateWebhookEndpointRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subscribed_events(
        mut self,
        value: Vec<UpdateWebhookEndpointRequestSubscribedEventsItem>,
    ) -> Self {
        self.subscribed_events = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookEndpointRequest`].
    pub fn build(self) -> Result<UpdateWebhookEndpointRequest, BuildError> {
        Ok(UpdateWebhookEndpointRequest {
            practice_ids: self.practice_ids,
            description: self.description,
            payload_style: self.payload_style,
            status: self.status,
            subscribed_events: self.subscribed_events,
            url: self.url,
        })
    }
}

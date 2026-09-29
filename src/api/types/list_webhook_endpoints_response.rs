pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookEndpointsResponse {
    #[serde(default)]
    pub data: Vec<ListWebhookEndpointsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListWebhookEndpointsResponseObject,
    pub url: ListWebhookEndpointsResponseUrl,
}

impl ListWebhookEndpointsResponse {
    pub fn builder() -> ListWebhookEndpointsResponseBuilder {
        <ListWebhookEndpointsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEndpointsResponseBuilder {
    data: Option<Vec<ListWebhookEndpointsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListWebhookEndpointsResponseObject>,
    url: Option<ListWebhookEndpointsResponseUrl>,
}

impl ListWebhookEndpointsResponseBuilder {
    pub fn data(mut self, value: Vec<ListWebhookEndpointsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListWebhookEndpointsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: ListWebhookEndpointsResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEndpointsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWebhookEndpointsResponseBuilder::data)
    /// - [`has_more`](ListWebhookEndpointsResponseBuilder::has_more)
    /// - [`object`](ListWebhookEndpointsResponseBuilder::object)
    /// - [`url`](ListWebhookEndpointsResponseBuilder::url)
    pub fn build(self) -> Result<ListWebhookEndpointsResponse, BuildError> {
        Ok(ListWebhookEndpointsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

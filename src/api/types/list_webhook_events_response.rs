pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookEventsResponse {
    #[serde(default)]
    pub data: Vec<ListWebhookEventsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListWebhookEventsResponseObject,
    pub url: ListWebhookEventsResponseUrl,
}

impl ListWebhookEventsResponse {
    pub fn builder() -> ListWebhookEventsResponseBuilder {
        <ListWebhookEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEventsResponseBuilder {
    data: Option<Vec<ListWebhookEventsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListWebhookEventsResponseObject>,
    url: Option<ListWebhookEventsResponseUrl>,
}

impl ListWebhookEventsResponseBuilder {
    pub fn data(mut self, value: Vec<ListWebhookEventsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListWebhookEventsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: ListWebhookEventsResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWebhookEventsResponseBuilder::data)
    /// - [`has_more`](ListWebhookEventsResponseBuilder::has_more)
    /// - [`object`](ListWebhookEventsResponseBuilder::object)
    /// - [`url`](ListWebhookEventsResponseBuilder::url)
    pub fn build(self) -> Result<ListWebhookEventsResponse, BuildError> {
        Ok(ListWebhookEventsResponse {
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

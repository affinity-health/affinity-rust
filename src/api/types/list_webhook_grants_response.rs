pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWebhookGrantsResponse {
    pub object: ListWebhookGrantsResponseObject,
    #[serde(default)]
    pub data: Vec<ListWebhookGrantsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub url: ListWebhookGrantsResponseUrl,
}

impl ListWebhookGrantsResponse {
    pub fn builder() -> ListWebhookGrantsResponseBuilder {
        <ListWebhookGrantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookGrantsResponseBuilder {
    object: Option<ListWebhookGrantsResponseObject>,
    data: Option<Vec<ListWebhookGrantsResponseDataItem>>,
    has_more: Option<bool>,
    url: Option<ListWebhookGrantsResponseUrl>,
}

impl ListWebhookGrantsResponseBuilder {
    pub fn object(mut self, value: ListWebhookGrantsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<ListWebhookGrantsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn url(mut self, value: ListWebhookGrantsResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookGrantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](ListWebhookGrantsResponseBuilder::object)
    /// - [`data`](ListWebhookGrantsResponseBuilder::data)
    /// - [`has_more`](ListWebhookGrantsResponseBuilder::has_more)
    /// - [`url`](ListWebhookGrantsResponseBuilder::url)
    pub fn build(self) -> Result<ListWebhookGrantsResponse, BuildError> {
        Ok(ListWebhookGrantsResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

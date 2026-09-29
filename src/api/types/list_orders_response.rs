pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrdersResponse {
    #[serde(default)]
    pub data: Vec<ListOrdersResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListOrdersResponseObject,
    pub url: ListOrdersResponseUrl,
}

impl ListOrdersResponse {
    pub fn builder() -> ListOrdersResponseBuilder {
        <ListOrdersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseBuilder {
    data: Option<Vec<ListOrdersResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListOrdersResponseObject>,
    url: Option<ListOrdersResponseUrl>,
}

impl ListOrdersResponseBuilder {
    pub fn data(mut self, value: Vec<ListOrdersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListOrdersResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: ListOrdersResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListOrdersResponseBuilder::data)
    /// - [`has_more`](ListOrdersResponseBuilder::has_more)
    /// - [`object`](ListOrdersResponseBuilder::object)
    /// - [`url`](ListOrdersResponseBuilder::url)
    pub fn build(self) -> Result<ListOrdersResponse, BuildError> {
        Ok(ListOrdersResponse {
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

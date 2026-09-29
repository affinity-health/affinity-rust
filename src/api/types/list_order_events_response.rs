pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrderEventsResponse {
    #[serde(default)]
    pub data: Vec<ListOrderEventsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListOrderEventsResponseObject,
    #[serde(default)]
    pub url: String,
}

impl ListOrderEventsResponse {
    pub fn builder() -> ListOrderEventsResponseBuilder {
        <ListOrderEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrderEventsResponseBuilder {
    data: Option<Vec<ListOrderEventsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListOrderEventsResponseObject>,
    url: Option<String>,
}

impl ListOrderEventsResponseBuilder {
    pub fn data(mut self, value: Vec<ListOrderEventsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListOrderEventsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrderEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListOrderEventsResponseBuilder::data)
    /// - [`has_more`](ListOrderEventsResponseBuilder::has_more)
    /// - [`object`](ListOrderEventsResponseBuilder::object)
    /// - [`url`](ListOrderEventsResponseBuilder::url)
    pub fn build(self) -> Result<ListOrderEventsResponse, BuildError> {
        Ok(ListOrderEventsResponse {
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

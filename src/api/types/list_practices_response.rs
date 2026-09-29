pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPracticesResponse {
    #[serde(default)]
    pub data: Vec<ListPracticesResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPracticesResponseObject,
    pub url: ListPracticesResponseUrl,
}

impl ListPracticesResponse {
    pub fn builder() -> ListPracticesResponseBuilder {
        <ListPracticesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticesResponseBuilder {
    data: Option<Vec<ListPracticesResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPracticesResponseObject>,
    url: Option<ListPracticesResponseUrl>,
}

impl ListPracticesResponseBuilder {
    pub fn data(mut self, value: Vec<ListPracticesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPracticesResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: ListPracticesResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPracticesResponseBuilder::data)
    /// - [`has_more`](ListPracticesResponseBuilder::has_more)
    /// - [`object`](ListPracticesResponseBuilder::object)
    /// - [`url`](ListPracticesResponseBuilder::url)
    pub fn build(self) -> Result<ListPracticesResponse, BuildError> {
        Ok(ListPracticesResponse {
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

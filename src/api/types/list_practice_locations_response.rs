pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPracticeLocationsResponse {
    #[serde(default)]
    pub data: Vec<ListPracticeLocationsResponseDataItem>,
    pub object: ListPracticeLocationsResponseObject,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub url: String,
}

impl ListPracticeLocationsResponse {
    pub fn builder() -> ListPracticeLocationsResponseBuilder {
        <ListPracticeLocationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeLocationsResponseBuilder {
    data: Option<Vec<ListPracticeLocationsResponseDataItem>>,
    object: Option<ListPracticeLocationsResponseObject>,
    has_more: Option<bool>,
    url: Option<String>,
}

impl ListPracticeLocationsResponseBuilder {
    pub fn data(mut self, value: Vec<ListPracticeLocationsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: ListPracticeLocationsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeLocationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPracticeLocationsResponseBuilder::data)
    /// - [`object`](ListPracticeLocationsResponseBuilder::object)
    /// - [`has_more`](ListPracticeLocationsResponseBuilder::has_more)
    /// - [`url`](ListPracticeLocationsResponseBuilder::url)
    pub fn build(self) -> Result<ListPracticeLocationsResponse, BuildError> {
        Ok(ListPracticeLocationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

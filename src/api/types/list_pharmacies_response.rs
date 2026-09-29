pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPharmaciesResponse {
    #[serde(default)]
    pub data: Vec<ListPharmaciesResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPharmaciesResponseObject,
    pub url: ListPharmaciesResponseUrl,
}

impl ListPharmaciesResponse {
    pub fn builder() -> ListPharmaciesResponseBuilder {
        <ListPharmaciesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPharmaciesResponseBuilder {
    data: Option<Vec<ListPharmaciesResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPharmaciesResponseObject>,
    url: Option<ListPharmaciesResponseUrl>,
}

impl ListPharmaciesResponseBuilder {
    pub fn data(mut self, value: Vec<ListPharmaciesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPharmaciesResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: ListPharmaciesResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPharmaciesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPharmaciesResponseBuilder::data)
    /// - [`has_more`](ListPharmaciesResponseBuilder::has_more)
    /// - [`object`](ListPharmaciesResponseBuilder::object)
    /// - [`url`](ListPharmaciesResponseBuilder::url)
    pub fn build(self) -> Result<ListPharmaciesResponse, BuildError> {
        Ok(ListPharmaciesResponse {
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

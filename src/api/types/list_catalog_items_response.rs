pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponse {
    #[serde(default)]
    pub data: Vec<ListCatalogItemsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListCatalogItemsResponseObject,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    pub url: ListCatalogItemsResponseUrl,
}

impl ListCatalogItemsResponse {
    pub fn builder() -> ListCatalogItemsResponseBuilder {
        <ListCatalogItemsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseBuilder {
    data: Option<Vec<ListCatalogItemsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListCatalogItemsResponseObject>,
    updated_at: Option<String>,
    url: Option<ListCatalogItemsResponseUrl>,
}

impl ListCatalogItemsResponseBuilder {
    pub fn data(mut self, value: Vec<ListCatalogItemsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListCatalogItemsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn url(mut self, value: ListCatalogItemsResponseUrl) -> Self {
        self.url = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCatalogItemsResponseBuilder::data)
    /// - [`has_more`](ListCatalogItemsResponseBuilder::has_more)
    /// - [`object`](ListCatalogItemsResponseBuilder::object)
    /// - [`updated_at`](ListCatalogItemsResponseBuilder::updated_at)
    /// - [`url`](ListCatalogItemsResponseBuilder::url)
    pub fn build(self) -> Result<ListCatalogItemsResponse, BuildError> {
        Ok(ListCatalogItemsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            has_more: self
                .has_more
                .ok_or_else(|| BuildError::missing_field("has_more"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

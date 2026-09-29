pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPatientAddressesResponse {
    #[serde(default)]
    pub data: Vec<ListPatientAddressesResponseDataItem>,
    pub object: ListPatientAddressesResponseObject,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub url: String,
}

impl ListPatientAddressesResponse {
    pub fn builder() -> ListPatientAddressesResponseBuilder {
        <ListPatientAddressesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientAddressesResponseBuilder {
    data: Option<Vec<ListPatientAddressesResponseDataItem>>,
    object: Option<ListPatientAddressesResponseObject>,
    has_more: Option<bool>,
    url: Option<String>,
}

impl ListPatientAddressesResponseBuilder {
    pub fn data(mut self, value: Vec<ListPatientAddressesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: ListPatientAddressesResponseObject) -> Self {
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

    /// Consumes the builder and constructs a [`ListPatientAddressesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPatientAddressesResponseBuilder::data)
    /// - [`object`](ListPatientAddressesResponseBuilder::object)
    /// - [`has_more`](ListPatientAddressesResponseBuilder::has_more)
    /// - [`url`](ListPatientAddressesResponseBuilder::url)
    pub fn build(self) -> Result<ListPatientAddressesResponse, BuildError> {
        Ok(ListPatientAddressesResponse {
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

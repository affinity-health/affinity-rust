pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPatientsResponse {
    #[serde(default)]
    pub data: Vec<ListPatientsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPatientsResponseObject,
    #[serde(default)]
    pub url: String,
}

impl ListPatientsResponse {
    pub fn builder() -> ListPatientsResponseBuilder {
        <ListPatientsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseBuilder {
    data: Option<Vec<ListPatientsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPatientsResponseObject>,
    url: Option<String>,
}

impl ListPatientsResponseBuilder {
    pub fn data(mut self, value: Vec<ListPatientsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPatientsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPatientsResponseBuilder::data)
    /// - [`has_more`](ListPatientsResponseBuilder::has_more)
    /// - [`object`](ListPatientsResponseBuilder::object)
    /// - [`url`](ListPatientsResponseBuilder::url)
    pub fn build(self) -> Result<ListPatientsResponse, BuildError> {
        Ok(ListPatientsResponse {
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

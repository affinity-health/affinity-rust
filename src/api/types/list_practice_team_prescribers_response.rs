pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamPrescribersResponse {
    #[serde(default)]
    pub data: Vec<ListPracticeTeamPrescribersResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPracticeTeamPrescribersResponseObject,
    #[serde(default)]
    pub url: String,
}

impl ListPracticeTeamPrescribersResponse {
    pub fn builder() -> ListPracticeTeamPrescribersResponseBuilder {
        <ListPracticeTeamPrescribersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamPrescribersResponseBuilder {
    data: Option<Vec<ListPracticeTeamPrescribersResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPracticeTeamPrescribersResponseObject>,
    url: Option<String>,
}

impl ListPracticeTeamPrescribersResponseBuilder {
    pub fn data(mut self, value: Vec<ListPracticeTeamPrescribersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPracticeTeamPrescribersResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamPrescribersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPracticeTeamPrescribersResponseBuilder::data)
    /// - [`has_more`](ListPracticeTeamPrescribersResponseBuilder::has_more)
    /// - [`object`](ListPracticeTeamPrescribersResponseBuilder::object)
    /// - [`url`](ListPracticeTeamPrescribersResponseBuilder::url)
    pub fn build(self) -> Result<ListPracticeTeamPrescribersResponse, BuildError> {
        Ok(ListPracticeTeamPrescribersResponse {
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

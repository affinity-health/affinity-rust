pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamMembersResponse {
    #[serde(default)]
    pub data: Vec<ListPracticeTeamMembersResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPracticeTeamMembersResponseObject,
    #[serde(default)]
    pub url: String,
}

impl ListPracticeTeamMembersResponse {
    pub fn builder() -> ListPracticeTeamMembersResponseBuilder {
        <ListPracticeTeamMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamMembersResponseBuilder {
    data: Option<Vec<ListPracticeTeamMembersResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPracticeTeamMembersResponseObject>,
    url: Option<String>,
}

impl ListPracticeTeamMembersResponseBuilder {
    pub fn data(mut self, value: Vec<ListPracticeTeamMembersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPracticeTeamMembersResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamMembersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPracticeTeamMembersResponseBuilder::data)
    /// - [`has_more`](ListPracticeTeamMembersResponseBuilder::has_more)
    /// - [`object`](ListPracticeTeamMembersResponseBuilder::object)
    /// - [`url`](ListPracticeTeamMembersResponseBuilder::url)
    pub fn build(self) -> Result<ListPracticeTeamMembersResponse, BuildError> {
        Ok(ListPracticeTeamMembersResponse {
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

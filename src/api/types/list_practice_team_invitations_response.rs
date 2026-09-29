pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsResponse {
    #[serde(default)]
    pub data: Vec<ListPracticeTeamInvitationsResponseDataItem>,
    #[serde(rename = "hasMore")]
    #[serde(default)]
    pub has_more: bool,
    pub object: ListPracticeTeamInvitationsResponseObject,
    #[serde(default)]
    pub url: String,
}

impl ListPracticeTeamInvitationsResponse {
    pub fn builder() -> ListPracticeTeamInvitationsResponseBuilder {
        <ListPracticeTeamInvitationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsResponseBuilder {
    data: Option<Vec<ListPracticeTeamInvitationsResponseDataItem>>,
    has_more: Option<bool>,
    object: Option<ListPracticeTeamInvitationsResponseObject>,
    url: Option<String>,
}

impl ListPracticeTeamInvitationsResponseBuilder {
    pub fn data(mut self, value: Vec<ListPracticeTeamInvitationsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    pub fn object(mut self, value: ListPracticeTeamInvitationsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPracticeTeamInvitationsResponseBuilder::data)
    /// - [`has_more`](ListPracticeTeamInvitationsResponseBuilder::has_more)
    /// - [`object`](ListPracticeTeamInvitationsResponseBuilder::object)
    /// - [`url`](ListPracticeTeamInvitationsResponseBuilder::url)
    pub fn build(self) -> Result<ListPracticeTeamInvitationsResponse, BuildError> {
        Ok(ListPracticeTeamInvitationsResponse {
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

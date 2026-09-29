pub use crate::prelude::*;

/// Query parameters for listPracticeTeamInvitations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPracticeTeamInvitationsRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Match this integration's external identity in the API key's mode.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl ListPracticeTeamInvitationsQueryRequest {
    pub fn builder() -> ListPracticeTeamInvitationsQueryRequestBuilder {
        <ListPracticeTeamInvitationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    status: Option<ListPracticeTeamInvitationsRequestStatus>,
    email: Option<String>,
    external_id: Option<String>,
}

impl ListPracticeTeamInvitationsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPracticeTeamInvitationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsQueryRequest`].
    pub fn build(self) -> Result<ListPracticeTeamInvitationsQueryRequest, BuildError> {
        Ok(ListPracticeTeamInvitationsQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            status: self.status,
            email: self.email,
            external_id: self.external_id,
        })
    }
}

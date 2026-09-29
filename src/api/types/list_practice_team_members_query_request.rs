pub use crate::prelude::*;

/// Query parameters for listPracticeTeamMembers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamMembersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ListPracticeTeamMembersRequestRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPracticeTeamMembersRequestStatus>,
}

impl ListPracticeTeamMembersQueryRequest {
    pub fn builder() -> ListPracticeTeamMembersQueryRequestBuilder {
        <ListPracticeTeamMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamMembersQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    search: Option<String>,
    role: Option<ListPracticeTeamMembersRequestRole>,
    status: Option<ListPracticeTeamMembersRequestStatus>,
}

impl ListPracticeTeamMembersQueryRequestBuilder {
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

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn role(mut self, value: ListPracticeTeamMembersRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn status(mut self, value: ListPracticeTeamMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamMembersQueryRequest`].
    pub fn build(self) -> Result<ListPracticeTeamMembersQueryRequest, BuildError> {
        Ok(ListPracticeTeamMembersQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            search: self.search,
            role: self.role,
            status: self.status,
        })
    }
}

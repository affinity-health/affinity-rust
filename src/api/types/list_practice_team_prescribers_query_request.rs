pub use crate::prelude::*;

/// Query parameters for listPracticeTeamPrescribers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamPrescribersQueryRequest {
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
    pub npi: Option<String>,
    /// Match a submitted license jurisdiction. This does not establish signing eligibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPracticeTeamPrescribersRequestStatus>,
}

impl ListPracticeTeamPrescribersQueryRequest {
    pub fn builder() -> ListPracticeTeamPrescribersQueryRequestBuilder {
        <ListPracticeTeamPrescribersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamPrescribersQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    search: Option<String>,
    npi: Option<String>,
    state: Option<String>,
    status: Option<ListPracticeTeamPrescribersRequestStatus>,
}

impl ListPracticeTeamPrescribersQueryRequestBuilder {
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

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPracticeTeamPrescribersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamPrescribersQueryRequest`].
    pub fn build(self) -> Result<ListPracticeTeamPrescribersQueryRequest, BuildError> {
        Ok(ListPracticeTeamPrescribersQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            search: self.search,
            npi: self.npi,
            state: self.state,
            status: self.status,
        })
    }
}

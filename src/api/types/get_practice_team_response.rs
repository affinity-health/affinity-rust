pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPracticeTeamResponse {
    pub object: GetPracticeTeamResponseObject,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    pub members: GetPracticeTeamResponseMembers,
    pub invitations: GetPracticeTeamResponseInvitations,
    pub prescribers: GetPracticeTeamResponsePrescribers,
}

impl GetPracticeTeamResponse {
    pub fn builder() -> GetPracticeTeamResponseBuilder {
        <GetPracticeTeamResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamResponseBuilder {
    object: Option<GetPracticeTeamResponseObject>,
    practice_id: Option<String>,
    members: Option<GetPracticeTeamResponseMembers>,
    invitations: Option<GetPracticeTeamResponseInvitations>,
    prescribers: Option<GetPracticeTeamResponsePrescribers>,
}

impl GetPracticeTeamResponseBuilder {
    pub fn object(mut self, value: GetPracticeTeamResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn members(mut self, value: GetPracticeTeamResponseMembers) -> Self {
        self.members = Some(value);
        self
    }

    pub fn invitations(mut self, value: GetPracticeTeamResponseInvitations) -> Self {
        self.invitations = Some(value);
        self
    }

    pub fn prescribers(mut self, value: GetPracticeTeamResponsePrescribers) -> Self {
        self.prescribers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](GetPracticeTeamResponseBuilder::object)
    /// - [`practice_id`](GetPracticeTeamResponseBuilder::practice_id)
    /// - [`members`](GetPracticeTeamResponseBuilder::members)
    /// - [`invitations`](GetPracticeTeamResponseBuilder::invitations)
    /// - [`prescribers`](GetPracticeTeamResponseBuilder::prescribers)
    pub fn build(self) -> Result<GetPracticeTeamResponse, BuildError> {
        Ok(GetPracticeTeamResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            invitations: self
                .invitations
                .ok_or_else(|| BuildError::missing_field("invitations"))?,
            prescribers: self
                .prescribers
                .ok_or_else(|| BuildError::missing_field("prescribers"))?,
        })
    }
}

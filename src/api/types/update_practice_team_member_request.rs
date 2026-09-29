pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UpdatePracticeTeamMemberRequestRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<UpdatePracticeTeamMemberRequestRolesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdatePracticeTeamMemberRequestStatus>,
    /// Replace location access. An empty array grants access to all practice locations.
    #[serde(rename = "locationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
}

impl UpdatePracticeTeamMemberRequest {
    pub fn builder() -> UpdatePracticeTeamMemberRequestBuilder {
        <UpdatePracticeTeamMemberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamMemberRequestBuilder {
    role: Option<UpdatePracticeTeamMemberRequestRole>,
    roles: Option<Vec<UpdatePracticeTeamMemberRequestRolesItem>>,
    status: Option<UpdatePracticeTeamMemberRequestStatus>,
    location_ids: Option<Vec<String>>,
}

impl UpdatePracticeTeamMemberRequestBuilder {
    pub fn role(mut self, value: UpdatePracticeTeamMemberRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<UpdatePracticeTeamMemberRequestRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn status(mut self, value: UpdatePracticeTeamMemberRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamMemberRequest`].
    pub fn build(self) -> Result<UpdatePracticeTeamMemberRequest, BuildError> {
        Ok(UpdatePracticeTeamMemberRequest {
            role: self.role,
            roles: self.roles,
            status: self.status,
            location_ids: self.location_ids,
        })
    }
}

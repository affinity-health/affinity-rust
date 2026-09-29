pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamInvitationResponsePersonInvitation {
    #[serde(default)]
    pub id: String,
    pub status: GetPracticeTeamInvitationResponsePersonInvitationStatus,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub roles: Vec<GetPracticeTeamInvitationResponsePersonInvitationRolesItem>,
}

impl GetPracticeTeamInvitationResponsePersonInvitation {
    pub fn builder() -> GetPracticeTeamInvitationResponsePersonInvitationBuilder {
        <GetPracticeTeamInvitationResponsePersonInvitationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamInvitationResponsePersonInvitationBuilder {
    id: Option<String>,
    status: Option<GetPracticeTeamInvitationResponsePersonInvitationStatus>,
    expires_at: Option<String>,
    roles: Option<Vec<GetPracticeTeamInvitationResponsePersonInvitationRolesItem>>,
}

impl GetPracticeTeamInvitationResponsePersonInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: GetPracticeTeamInvitationResponsePersonInvitationStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn roles(
        mut self,
        value: Vec<GetPracticeTeamInvitationResponsePersonInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamInvitationResponsePersonInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPracticeTeamInvitationResponsePersonInvitationBuilder::id)
    /// - [`status`](GetPracticeTeamInvitationResponsePersonInvitationBuilder::status)
    /// - [`expires_at`](GetPracticeTeamInvitationResponsePersonInvitationBuilder::expires_at)
    /// - [`roles`](GetPracticeTeamInvitationResponsePersonInvitationBuilder::roles)
    pub fn build(self) -> Result<GetPracticeTeamInvitationResponsePersonInvitation, BuildError> {
        Ok(GetPracticeTeamInvitationResponsePersonInvitation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
        })
    }
}

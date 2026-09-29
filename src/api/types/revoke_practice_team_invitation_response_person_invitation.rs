pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevokePracticeTeamInvitationResponsePersonInvitation {
    #[serde(default)]
    pub id: String,
    pub status: RevokePracticeTeamInvitationResponsePersonInvitationStatus,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub roles: Vec<RevokePracticeTeamInvitationResponsePersonInvitationRolesItem>,
}

impl RevokePracticeTeamInvitationResponsePersonInvitation {
    pub fn builder() -> RevokePracticeTeamInvitationResponsePersonInvitationBuilder {
        <RevokePracticeTeamInvitationResponsePersonInvitationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePracticeTeamInvitationResponsePersonInvitationBuilder {
    id: Option<String>,
    status: Option<RevokePracticeTeamInvitationResponsePersonInvitationStatus>,
    expires_at: Option<String>,
    roles: Option<Vec<RevokePracticeTeamInvitationResponsePersonInvitationRolesItem>>,
}

impl RevokePracticeTeamInvitationResponsePersonInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: RevokePracticeTeamInvitationResponsePersonInvitationStatus,
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
        value: Vec<RevokePracticeTeamInvitationResponsePersonInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePracticeTeamInvitationResponsePersonInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokePracticeTeamInvitationResponsePersonInvitationBuilder::id)
    /// - [`status`](RevokePracticeTeamInvitationResponsePersonInvitationBuilder::status)
    /// - [`expires_at`](RevokePracticeTeamInvitationResponsePersonInvitationBuilder::expires_at)
    /// - [`roles`](RevokePracticeTeamInvitationResponsePersonInvitationBuilder::roles)
    pub fn build(self) -> Result<RevokePracticeTeamInvitationResponsePersonInvitation, BuildError> {
        Ok(RevokePracticeTeamInvitationResponsePersonInvitation {
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

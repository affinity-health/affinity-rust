pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponsePersonInvitation {
    #[serde(default)]
    pub id: String,
    pub status: InvitePracticeTeamPersonResponsePersonInvitationStatus,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub roles: Vec<InvitePracticeTeamPersonResponsePersonInvitationRolesItem>,
}

impl InvitePracticeTeamPersonResponsePersonInvitation {
    pub fn builder() -> InvitePracticeTeamPersonResponsePersonInvitationBuilder {
        <InvitePracticeTeamPersonResponsePersonInvitationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponsePersonInvitationBuilder {
    id: Option<String>,
    status: Option<InvitePracticeTeamPersonResponsePersonInvitationStatus>,
    expires_at: Option<String>,
    roles: Option<Vec<InvitePracticeTeamPersonResponsePersonInvitationRolesItem>>,
}

impl InvitePracticeTeamPersonResponsePersonInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: InvitePracticeTeamPersonResponsePersonInvitationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn roles(
        mut self,
        value: Vec<InvitePracticeTeamPersonResponsePersonInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponsePersonInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitePracticeTeamPersonResponsePersonInvitationBuilder::id)
    /// - [`status`](InvitePracticeTeamPersonResponsePersonInvitationBuilder::status)
    /// - [`expires_at`](InvitePracticeTeamPersonResponsePersonInvitationBuilder::expires_at)
    /// - [`roles`](InvitePracticeTeamPersonResponsePersonInvitationBuilder::roles)
    pub fn build(self) -> Result<InvitePracticeTeamPersonResponsePersonInvitation, BuildError> {
        Ok(InvitePracticeTeamPersonResponsePersonInvitation {
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

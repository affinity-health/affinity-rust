pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendPracticeTeamInvitationResponseInvitationPersonInvitation {
    #[serde(default)]
    pub id: String,
    pub status: ResendPracticeTeamInvitationResponseInvitationPersonInvitationStatus,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub roles: Vec<ResendPracticeTeamInvitationResponseInvitationPersonInvitationRolesItem>,
}

impl ResendPracticeTeamInvitationResponseInvitationPersonInvitation {
    pub fn builder() -> ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder {
        <ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder {
    id: Option<String>,
    status: Option<ResendPracticeTeamInvitationResponseInvitationPersonInvitationStatus>,
    expires_at: Option<String>,
    roles: Option<Vec<ResendPracticeTeamInvitationResponseInvitationPersonInvitationRolesItem>>,
}

impl ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: ResendPracticeTeamInvitationResponseInvitationPersonInvitationStatus,
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
        value: Vec<ResendPracticeTeamInvitationResponseInvitationPersonInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendPracticeTeamInvitationResponseInvitationPersonInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder::id)
    /// - [`status`](ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder::status)
    /// - [`expires_at`](ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder::expires_at)
    /// - [`roles`](ResendPracticeTeamInvitationResponseInvitationPersonInvitationBuilder::roles)
    pub fn build(
        self,
    ) -> Result<ResendPracticeTeamInvitationResponseInvitationPersonInvitation, BuildError> {
        Ok(
            ResendPracticeTeamInvitationResponseInvitationPersonInvitation {
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
            },
        )
    }
}

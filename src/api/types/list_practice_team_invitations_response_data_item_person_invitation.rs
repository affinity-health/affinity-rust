pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonInvitation {
    #[serde(default)]
    pub id: String,
    pub status: ListPracticeTeamInvitationsResponseDataItemPersonInvitationStatus,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub roles: Vec<ListPracticeTeamInvitationsResponseDataItemPersonInvitationRolesItem>,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonInvitation {
    pub fn builder() -> ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder {
        <ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder {
    id: Option<String>,
    status: Option<ListPracticeTeamInvitationsResponseDataItemPersonInvitationStatus>,
    expires_at: Option<String>,
    roles: Option<Vec<ListPracticeTeamInvitationsResponseDataItemPersonInvitationRolesItem>>,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: ListPracticeTeamInvitationsResponseDataItemPersonInvitationStatus,
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
        value: Vec<ListPracticeTeamInvitationsResponseDataItemPersonInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsResponseDataItemPersonInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder::id)
    /// - [`status`](ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder::status)
    /// - [`expires_at`](ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder::expires_at)
    /// - [`roles`](ListPracticeTeamInvitationsResponseDataItemPersonInvitationBuilder::roles)
    pub fn build(
        self,
    ) -> Result<ListPracticeTeamInvitationsResponseDataItemPersonInvitation, BuildError> {
        Ok(
            ListPracticeTeamInvitationsResponseDataItemPersonInvitation {
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

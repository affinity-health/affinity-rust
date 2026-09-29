pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendPracticeTeamInvitationResponseInvitation {
    #[serde(default)]
    pub id: String,
    pub object: ResendPracticeTeamInvitationResponseInvitationObject,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub status: ResendPracticeTeamInvitationResponseInvitationStatus,
    #[serde(default)]
    pub roles: Vec<ResendPracticeTeamInvitationResponseInvitationRolesItem>,
    #[serde(rename = "locationIds")]
    #[serde(default)]
    pub location_ids: Vec<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
    #[serde(rename = "acceptedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    /// This integration's mode-scoped user ID, used for draft attribution and sessions after acceptance. Null for invitations outside this integration.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "memberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "prescriberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_id: Option<String>,
    /// This integration's current onboarding and account-connection state. Null for invitations outside this integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<ResendPracticeTeamInvitationResponseInvitationPerson>,
}

impl ResendPracticeTeamInvitationResponseInvitation {
    pub fn builder() -> ResendPracticeTeamInvitationResponseInvitationBuilder {
        <ResendPracticeTeamInvitationResponseInvitationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendPracticeTeamInvitationResponseInvitationBuilder {
    id: Option<String>,
    object: Option<ResendPracticeTeamInvitationResponseInvitationObject>,
    email: Option<String>,
    name: Option<String>,
    status: Option<ResendPracticeTeamInvitationResponseInvitationStatus>,
    roles: Option<Vec<ResendPracticeTeamInvitationResponseInvitationRolesItem>>,
    location_ids: Option<Vec<String>>,
    created_at: Option<String>,
    expires_at: Option<String>,
    accepted_at: Option<String>,
    user_id: Option<String>,
    external_id: Option<String>,
    member_id: Option<String>,
    prescriber_id: Option<String>,
    person: Option<ResendPracticeTeamInvitationResponseInvitationPerson>,
}

impl ResendPracticeTeamInvitationResponseInvitationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ResendPracticeTeamInvitationResponseInvitationObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResendPracticeTeamInvitationResponseInvitationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn roles(
        mut self,
        value: Vec<ResendPracticeTeamInvitationResponseInvitationRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn accepted_at(mut self, value: impl Into<String>) -> Self {
        self.accepted_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn prescriber_id(mut self, value: impl Into<String>) -> Self {
        self.prescriber_id = Some(value.into());
        self
    }

    pub fn person(mut self, value: ResendPracticeTeamInvitationResponseInvitationPerson) -> Self {
        self.person = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendPracticeTeamInvitationResponseInvitation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResendPracticeTeamInvitationResponseInvitationBuilder::id)
    /// - [`object`](ResendPracticeTeamInvitationResponseInvitationBuilder::object)
    /// - [`email`](ResendPracticeTeamInvitationResponseInvitationBuilder::email)
    /// - [`status`](ResendPracticeTeamInvitationResponseInvitationBuilder::status)
    /// - [`roles`](ResendPracticeTeamInvitationResponseInvitationBuilder::roles)
    /// - [`location_ids`](ResendPracticeTeamInvitationResponseInvitationBuilder::location_ids)
    /// - [`created_at`](ResendPracticeTeamInvitationResponseInvitationBuilder::created_at)
    /// - [`expires_at`](ResendPracticeTeamInvitationResponseInvitationBuilder::expires_at)
    pub fn build(self) -> Result<ResendPracticeTeamInvitationResponseInvitation, BuildError> {
        Ok(ResendPracticeTeamInvitationResponseInvitation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            location_ids: self
                .location_ids
                .ok_or_else(|| BuildError::missing_field("location_ids"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            accepted_at: self.accepted_at,
            user_id: self.user_id,
            external_id: self.external_id,
            member_id: self.member_id,
            prescriber_id: self.prescriber_id,
            person: self.person,
        })
    }
}

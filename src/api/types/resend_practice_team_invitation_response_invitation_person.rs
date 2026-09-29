pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendPracticeTeamInvitationResponseInvitationPerson {
    #[serde(default)]
    pub id: String,
    pub object: ResendPracticeTeamInvitationResponseInvitationPersonObject,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation: Option<ResendPracticeTeamInvitationResponseInvitationPersonInvitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<ResendPracticeTeamInvitationResponseInvitationPersonAccount>,
    #[serde(rename = "nextActions")]
    #[serde(default)]
    pub next_actions: Vec<String>,
}

impl ResendPracticeTeamInvitationResponseInvitationPerson {
    pub fn builder() -> ResendPracticeTeamInvitationResponseInvitationPersonBuilder {
        <ResendPracticeTeamInvitationResponseInvitationPersonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendPracticeTeamInvitationResponseInvitationPersonBuilder {
    id: Option<String>,
    object: Option<ResendPracticeTeamInvitationResponseInvitationPersonObject>,
    external_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    status: Option<String>,
    invitation: Option<ResendPracticeTeamInvitationResponseInvitationPersonInvitation>,
    account: Option<ResendPracticeTeamInvitationResponseInvitationPersonAccount>,
    next_actions: Option<Vec<String>>,
}

impl ResendPracticeTeamInvitationResponseInvitationPersonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(
        mut self,
        value: ResendPracticeTeamInvitationResponseInvitationPersonObject,
    ) -> Self {
        self.object = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
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

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn invitation(
        mut self,
        value: ResendPracticeTeamInvitationResponseInvitationPersonInvitation,
    ) -> Self {
        self.invitation = Some(value);
        self
    }

    pub fn account(
        mut self,
        value: ResendPracticeTeamInvitationResponseInvitationPersonAccount,
    ) -> Self {
        self.account = Some(value);
        self
    }

    pub fn next_actions(mut self, value: Vec<String>) -> Self {
        self.next_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendPracticeTeamInvitationResponseInvitationPerson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResendPracticeTeamInvitationResponseInvitationPersonBuilder::id)
    /// - [`object`](ResendPracticeTeamInvitationResponseInvitationPersonBuilder::object)
    /// - [`external_id`](ResendPracticeTeamInvitationResponseInvitationPersonBuilder::external_id)
    /// - [`status`](ResendPracticeTeamInvitationResponseInvitationPersonBuilder::status)
    /// - [`next_actions`](ResendPracticeTeamInvitationResponseInvitationPersonBuilder::next_actions)
    pub fn build(self) -> Result<ResendPracticeTeamInvitationResponseInvitationPerson, BuildError> {
        Ok(ResendPracticeTeamInvitationResponseInvitationPerson {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            email: self.email,
            name: self.name,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            invitation: self.invitation,
            account: self.account,
            next_actions: self
                .next_actions
                .ok_or_else(|| BuildError::missing_field("next_actions"))?,
        })
    }
}

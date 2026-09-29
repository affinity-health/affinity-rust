pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevokePracticeTeamInvitationResponsePerson {
    #[serde(default)]
    pub id: String,
    pub object: RevokePracticeTeamInvitationResponsePersonObject,
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
    pub invitation: Option<RevokePracticeTeamInvitationResponsePersonInvitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<RevokePracticeTeamInvitationResponsePersonAccount>,
    #[serde(rename = "nextActions")]
    #[serde(default)]
    pub next_actions: Vec<String>,
}

impl RevokePracticeTeamInvitationResponsePerson {
    pub fn builder() -> RevokePracticeTeamInvitationResponsePersonBuilder {
        <RevokePracticeTeamInvitationResponsePersonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePracticeTeamInvitationResponsePersonBuilder {
    id: Option<String>,
    object: Option<RevokePracticeTeamInvitationResponsePersonObject>,
    external_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    status: Option<String>,
    invitation: Option<RevokePracticeTeamInvitationResponsePersonInvitation>,
    account: Option<RevokePracticeTeamInvitationResponsePersonAccount>,
    next_actions: Option<Vec<String>>,
}

impl RevokePracticeTeamInvitationResponsePersonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: RevokePracticeTeamInvitationResponsePersonObject) -> Self {
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
        value: RevokePracticeTeamInvitationResponsePersonInvitation,
    ) -> Self {
        self.invitation = Some(value);
        self
    }

    pub fn account(mut self, value: RevokePracticeTeamInvitationResponsePersonAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn next_actions(mut self, value: Vec<String>) -> Self {
        self.next_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePracticeTeamInvitationResponsePerson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokePracticeTeamInvitationResponsePersonBuilder::id)
    /// - [`object`](RevokePracticeTeamInvitationResponsePersonBuilder::object)
    /// - [`external_id`](RevokePracticeTeamInvitationResponsePersonBuilder::external_id)
    /// - [`status`](RevokePracticeTeamInvitationResponsePersonBuilder::status)
    /// - [`next_actions`](RevokePracticeTeamInvitationResponsePersonBuilder::next_actions)
    pub fn build(self) -> Result<RevokePracticeTeamInvitationResponsePerson, BuildError> {
        Ok(RevokePracticeTeamInvitationResponsePerson {
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

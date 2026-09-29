pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponsePerson {
    #[serde(default)]
    pub id: String,
    pub object: InvitePracticeTeamPersonResponsePersonObject,
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
    pub invitation: Option<InvitePracticeTeamPersonResponsePersonInvitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<InvitePracticeTeamPersonResponsePersonAccount>,
    #[serde(rename = "nextActions")]
    #[serde(default)]
    pub next_actions: Vec<String>,
}

impl InvitePracticeTeamPersonResponsePerson {
    pub fn builder() -> InvitePracticeTeamPersonResponsePersonBuilder {
        <InvitePracticeTeamPersonResponsePersonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponsePersonBuilder {
    id: Option<String>,
    object: Option<InvitePracticeTeamPersonResponsePersonObject>,
    external_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    status: Option<String>,
    invitation: Option<InvitePracticeTeamPersonResponsePersonInvitation>,
    account: Option<InvitePracticeTeamPersonResponsePersonAccount>,
    next_actions: Option<Vec<String>>,
}

impl InvitePracticeTeamPersonResponsePersonBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: InvitePracticeTeamPersonResponsePersonObject) -> Self {
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

    pub fn invitation(mut self, value: InvitePracticeTeamPersonResponsePersonInvitation) -> Self {
        self.invitation = Some(value);
        self
    }

    pub fn account(mut self, value: InvitePracticeTeamPersonResponsePersonAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn next_actions(mut self, value: Vec<String>) -> Self {
        self.next_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponsePerson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitePracticeTeamPersonResponsePersonBuilder::id)
    /// - [`object`](InvitePracticeTeamPersonResponsePersonBuilder::object)
    /// - [`external_id`](InvitePracticeTeamPersonResponsePersonBuilder::external_id)
    /// - [`status`](InvitePracticeTeamPersonResponsePersonBuilder::status)
    /// - [`next_actions`](InvitePracticeTeamPersonResponsePersonBuilder::next_actions)
    pub fn build(self) -> Result<InvitePracticeTeamPersonResponsePerson, BuildError> {
        Ok(InvitePracticeTeamPersonResponsePerson {
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

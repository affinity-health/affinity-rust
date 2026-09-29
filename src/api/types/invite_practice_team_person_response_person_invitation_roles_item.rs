pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponsePersonInvitationRolesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl InvitePracticeTeamPersonResponsePersonInvitationRolesItem {
    pub fn builder() -> InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder {
        <InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    key: Option<String>,
}

impl InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponsePersonInvitationRolesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder::id)
    /// - [`name`](InvitePracticeTeamPersonResponsePersonInvitationRolesItemBuilder::name)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonResponsePersonInvitationRolesItem, BuildError> {
        Ok(InvitePracticeTeamPersonResponsePersonInvitationRolesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            key: self.key,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePracticeTeamInvitationResponseRolesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl RevokePracticeTeamInvitationResponseRolesItem {
    pub fn builder() -> RevokePracticeTeamInvitationResponseRolesItemBuilder {
        <RevokePracticeTeamInvitationResponseRolesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePracticeTeamInvitationResponseRolesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    key: Option<String>,
}

impl RevokePracticeTeamInvitationResponseRolesItemBuilder {
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

    /// Consumes the builder and constructs a [`RevokePracticeTeamInvitationResponseRolesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokePracticeTeamInvitationResponseRolesItemBuilder::id)
    /// - [`name`](RevokePracticeTeamInvitationResponseRolesItemBuilder::name)
    pub fn build(self) -> Result<RevokePracticeTeamInvitationResponseRolesItem, BuildError> {
        Ok(RevokePracticeTeamInvitationResponseRolesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            key: self.key,
        })
    }
}

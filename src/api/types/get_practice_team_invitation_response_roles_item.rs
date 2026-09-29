pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamInvitationResponseRolesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl GetPracticeTeamInvitationResponseRolesItem {
    pub fn builder() -> GetPracticeTeamInvitationResponseRolesItemBuilder {
        <GetPracticeTeamInvitationResponseRolesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamInvitationResponseRolesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    key: Option<String>,
}

impl GetPracticeTeamInvitationResponseRolesItemBuilder {
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

    /// Consumes the builder and constructs a [`GetPracticeTeamInvitationResponseRolesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPracticeTeamInvitationResponseRolesItemBuilder::id)
    /// - [`name`](GetPracticeTeamInvitationResponseRolesItemBuilder::name)
    pub fn build(self) -> Result<GetPracticeTeamInvitationResponseRolesItem, BuildError> {
        Ok(GetPracticeTeamInvitationResponseRolesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            key: self.key,
        })
    }
}

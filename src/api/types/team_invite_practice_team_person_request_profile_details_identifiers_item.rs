pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem {
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub description: String,
}

impl InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem {
    pub fn builder() -> InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder {
        <InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder {
    identifier: Option<String>,
    issuer: Option<String>,
    state: Option<String>,
    description: Option<String>,
}

impl InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder {
    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`identifier`](InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder::identifier)
    /// - [`issuer`](InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder::issuer)
    /// - [`state`](InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder::state)
    /// - [`description`](InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItemBuilder::description)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem, BuildError> {
        Ok(
            InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem {
                identifier: self
                    .identifier
                    .ok_or_else(|| BuildError::missing_field("identifier"))?,
                issuer: self
                    .issuer
                    .ok_or_else(|| BuildError::missing_field("issuer"))?,
                state: self
                    .state
                    .ok_or_else(|| BuildError::missing_field("state"))?,
                description: self
                    .description
                    .ok_or_else(|| BuildError::missing_field("description"))?,
            },
        )
    }
}

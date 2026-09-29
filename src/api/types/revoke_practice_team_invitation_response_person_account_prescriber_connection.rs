pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider,
}

impl RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection {
    pub fn builder() -> RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder
    {
        <RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider>,
}

impl RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection, BuildError>
    {
        Ok(
            RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection {
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
                provider: self
                    .provider
                    .ok_or_else(|| BuildError::missing_field("provider"))?,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamInvitationResponsePersonAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider,
}

impl GetPracticeTeamInvitationResponsePersonAccountPrescriberConnection {
    pub fn builder() -> GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder {
        <GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider>,
}

impl GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamInvitationResponsePersonAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<GetPracticeTeamInvitationResponsePersonAccountPrescriberConnection, BuildError>
    {
        Ok(
            GetPracticeTeamInvitationResponsePersonAccountPrescriberConnection {
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

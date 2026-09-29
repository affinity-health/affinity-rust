pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider:
        ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionProvider,
}

impl ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnection {
    pub fn builder(
    ) -> ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder
    {
        <ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<
        ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionProvider,
    >,
}

impl ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<
        ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnection,
        BuildError,
    > {
        Ok(
            ResendPracticeTeamInvitationResponseInvitationPersonAccountPrescriberConnection {
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

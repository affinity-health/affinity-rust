pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponsePersonAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider,
}

impl InvitePracticeTeamPersonResponsePersonAccountPrescriberConnection {
    pub fn builder() -> InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder {
        <InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider>,
}

impl InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponsePersonAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnection, BuildError> {
        Ok(
            InvitePracticeTeamPersonResponsePersonAccountPrescriberConnection {
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

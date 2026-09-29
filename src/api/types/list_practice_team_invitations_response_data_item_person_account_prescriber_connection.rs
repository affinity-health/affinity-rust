pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider:
        ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProvider,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection {
    pub fn builder(
    ) -> ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder {
        <ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<
        ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProvider,
    >,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<
        ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection,
        BuildError,
    > {
        Ok(
            ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection {
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

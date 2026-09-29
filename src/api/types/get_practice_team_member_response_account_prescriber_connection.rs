pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamMemberResponseAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: GetPracticeTeamMemberResponseAccountPrescriberConnectionProvider,
}

impl GetPracticeTeamMemberResponseAccountPrescriberConnection {
    pub fn builder() -> GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
        <GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<GetPracticeTeamMemberResponseAccountPrescriberConnectionProvider>,
}

impl GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: GetPracticeTeamMemberResponseAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamMemberResponseAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](GetPracticeTeamMemberResponseAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<GetPracticeTeamMemberResponseAccountPrescriberConnection, BuildError> {
        Ok(GetPracticeTeamMemberResponseAccountPrescriberConnection {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
        })
    }
}

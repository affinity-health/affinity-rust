pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamMemberResponseAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider,
}

impl UpdatePracticeTeamMemberResponseAccountPrescriberConnection {
    pub fn builder() -> UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
        <UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider>,
}

impl UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamMemberResponseAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<UpdatePracticeTeamMemberResponseAccountPrescriberConnection, BuildError> {
        Ok(
            UpdatePracticeTeamMemberResponseAccountPrescriberConnection {
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

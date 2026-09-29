pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamMembersResponseDataItemAccountPrescriberConnection {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub provider: ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionProvider,
}

impl ListPracticeTeamMembersResponseDataItemAccountPrescriberConnection {
    pub fn builder() -> ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder {
        <ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder {
    status: Option<String>,
    provider: Option<ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionProvider>,
}

impl ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn provider(
        mut self,
        value: ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionProvider,
    ) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamMembersResponseDataItemAccountPrescriberConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder::status)
    /// - [`provider`](ListPracticeTeamMembersResponseDataItemAccountPrescriberConnectionBuilder::provider)
    pub fn build(
        self,
    ) -> Result<ListPracticeTeamMembersResponseDataItemAccountPrescriberConnection, BuildError>
    {
        Ok(
            ListPracticeTeamMembersResponseDataItemAccountPrescriberConnection {
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

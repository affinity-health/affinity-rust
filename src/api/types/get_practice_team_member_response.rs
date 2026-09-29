pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamMemberResponse {
    #[serde(default)]
    pub id: String,
    /// This integration's external ID for the accepted invitee in the API key's mode. Null for members invited outside this integration.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "locationIds")]
    #[serde(default)]
    pub location_ids: Vec<String>,
    #[serde(default)]
    pub account: GetPracticeTeamMemberResponseAccount,
    #[serde(rename = "nextActions")]
    #[serde(default)]
    pub next_actions: Vec<String>,
}

impl GetPracticeTeamMemberResponse {
    pub fn builder() -> GetPracticeTeamMemberResponseBuilder {
        <GetPracticeTeamMemberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamMemberResponseBuilder {
    id: Option<String>,
    external_id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    location_ids: Option<Vec<String>>,
    account: Option<GetPracticeTeamMemberResponseAccount>,
    next_actions: Option<Vec<String>>,
}

impl GetPracticeTeamMemberResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    pub fn account(mut self, value: GetPracticeTeamMemberResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn next_actions(mut self, value: Vec<String>) -> Self {
        self.next_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamMemberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPracticeTeamMemberResponseBuilder::id)
    /// - [`name`](GetPracticeTeamMemberResponseBuilder::name)
    /// - [`location_ids`](GetPracticeTeamMemberResponseBuilder::location_ids)
    /// - [`account`](GetPracticeTeamMemberResponseBuilder::account)
    /// - [`next_actions`](GetPracticeTeamMemberResponseBuilder::next_actions)
    pub fn build(self) -> Result<GetPracticeTeamMemberResponse, BuildError> {
        Ok(GetPracticeTeamMemberResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_id: self.external_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            email: self.email,
            location_ids: self
                .location_ids
                .ok_or_else(|| BuildError::missing_field("location_ids"))?,
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            next_actions: self
                .next_actions
                .ok_or_else(|| BuildError::missing_field("next_actions"))?,
        })
    }
}

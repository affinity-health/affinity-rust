pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamMemberResponseAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "emailVerified")]
    #[serde(default)]
    pub email_verified: bool,
    #[serde(rename = "membershipId")]
    #[serde(default)]
    pub membership_id: String,
    #[serde(rename = "membershipStatus")]
    #[serde(default)]
    pub membership_status: String,
    #[serde(default)]
    pub roles: Vec<GetPracticeTeamMemberResponseAccountRolesItem>,
    #[serde(rename = "prescriberConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_connection: Option<GetPracticeTeamMemberResponseAccountPrescriberConnection>,
}

impl GetPracticeTeamMemberResponseAccount {
    pub fn builder() -> GetPracticeTeamMemberResponseAccountBuilder {
        <GetPracticeTeamMemberResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamMemberResponseAccountBuilder {
    account_id: Option<String>,
    email_verified: Option<bool>,
    membership_id: Option<String>,
    membership_status: Option<String>,
    roles: Option<Vec<GetPracticeTeamMemberResponseAccountRolesItem>>,
    prescriber_connection: Option<GetPracticeTeamMemberResponseAccountPrescriberConnection>,
}

impl GetPracticeTeamMemberResponseAccountBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = Some(value);
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn membership_status(mut self, value: impl Into<String>) -> Self {
        self.membership_status = Some(value.into());
        self
    }

    pub fn roles(mut self, value: Vec<GetPracticeTeamMemberResponseAccountRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn prescriber_connection(
        mut self,
        value: GetPracticeTeamMemberResponseAccountPrescriberConnection,
    ) -> Self {
        self.prescriber_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamMemberResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](GetPracticeTeamMemberResponseAccountBuilder::account_id)
    /// - [`email_verified`](GetPracticeTeamMemberResponseAccountBuilder::email_verified)
    /// - [`membership_id`](GetPracticeTeamMemberResponseAccountBuilder::membership_id)
    /// - [`membership_status`](GetPracticeTeamMemberResponseAccountBuilder::membership_status)
    /// - [`roles`](GetPracticeTeamMemberResponseAccountBuilder::roles)
    pub fn build(self) -> Result<GetPracticeTeamMemberResponseAccount, BuildError> {
        Ok(GetPracticeTeamMemberResponseAccount {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            email_verified: self
                .email_verified
                .ok_or_else(|| BuildError::missing_field("email_verified"))?,
            membership_id: self
                .membership_id
                .ok_or_else(|| BuildError::missing_field("membership_id"))?,
            membership_status: self
                .membership_status
                .ok_or_else(|| BuildError::missing_field("membership_status"))?,
            roles: self
                .roles
                .ok_or_else(|| BuildError::missing_field("roles"))?,
            prescriber_connection: self.prescriber_connection,
        })
    }
}

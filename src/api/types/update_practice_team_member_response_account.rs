pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamMemberResponseAccount {
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
    pub roles: Vec<UpdatePracticeTeamMemberResponseAccountRolesItem>,
    #[serde(rename = "prescriberConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_connection: Option<UpdatePracticeTeamMemberResponseAccountPrescriberConnection>,
}

impl UpdatePracticeTeamMemberResponseAccount {
    pub fn builder() -> UpdatePracticeTeamMemberResponseAccountBuilder {
        <UpdatePracticeTeamMemberResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamMemberResponseAccountBuilder {
    account_id: Option<String>,
    email_verified: Option<bool>,
    membership_id: Option<String>,
    membership_status: Option<String>,
    roles: Option<Vec<UpdatePracticeTeamMemberResponseAccountRolesItem>>,
    prescriber_connection: Option<UpdatePracticeTeamMemberResponseAccountPrescriberConnection>,
}

impl UpdatePracticeTeamMemberResponseAccountBuilder {
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

    pub fn roles(mut self, value: Vec<UpdatePracticeTeamMemberResponseAccountRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn prescriber_connection(
        mut self,
        value: UpdatePracticeTeamMemberResponseAccountPrescriberConnection,
    ) -> Self {
        self.prescriber_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamMemberResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](UpdatePracticeTeamMemberResponseAccountBuilder::account_id)
    /// - [`email_verified`](UpdatePracticeTeamMemberResponseAccountBuilder::email_verified)
    /// - [`membership_id`](UpdatePracticeTeamMemberResponseAccountBuilder::membership_id)
    /// - [`membership_status`](UpdatePracticeTeamMemberResponseAccountBuilder::membership_status)
    /// - [`roles`](UpdatePracticeTeamMemberResponseAccountBuilder::roles)
    pub fn build(self) -> Result<UpdatePracticeTeamMemberResponseAccount, BuildError> {
        Ok(UpdatePracticeTeamMemberResponseAccount {
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

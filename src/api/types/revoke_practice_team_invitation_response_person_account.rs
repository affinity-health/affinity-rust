pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePracticeTeamInvitationResponsePersonAccount {
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
    pub roles: Vec<RevokePracticeTeamInvitationResponsePersonAccountRolesItem>,
    #[serde(rename = "prescriberConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_connection:
        Option<RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection>,
}

impl RevokePracticeTeamInvitationResponsePersonAccount {
    pub fn builder() -> RevokePracticeTeamInvitationResponsePersonAccountBuilder {
        <RevokePracticeTeamInvitationResponsePersonAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePracticeTeamInvitationResponsePersonAccountBuilder {
    account_id: Option<String>,
    email_verified: Option<bool>,
    membership_id: Option<String>,
    membership_status: Option<String>,
    roles: Option<Vec<RevokePracticeTeamInvitationResponsePersonAccountRolesItem>>,
    prescriber_connection:
        Option<RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection>,
}

impl RevokePracticeTeamInvitationResponsePersonAccountBuilder {
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

    pub fn roles(
        mut self,
        value: Vec<RevokePracticeTeamInvitationResponsePersonAccountRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn prescriber_connection(
        mut self,
        value: RevokePracticeTeamInvitationResponsePersonAccountPrescriberConnection,
    ) -> Self {
        self.prescriber_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokePracticeTeamInvitationResponsePersonAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](RevokePracticeTeamInvitationResponsePersonAccountBuilder::account_id)
    /// - [`email_verified`](RevokePracticeTeamInvitationResponsePersonAccountBuilder::email_verified)
    /// - [`membership_id`](RevokePracticeTeamInvitationResponsePersonAccountBuilder::membership_id)
    /// - [`membership_status`](RevokePracticeTeamInvitationResponsePersonAccountBuilder::membership_status)
    /// - [`roles`](RevokePracticeTeamInvitationResponsePersonAccountBuilder::roles)
    pub fn build(self) -> Result<RevokePracticeTeamInvitationResponsePersonAccount, BuildError> {
        Ok(RevokePracticeTeamInvitationResponsePersonAccount {
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

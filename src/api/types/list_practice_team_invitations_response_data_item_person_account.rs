pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccount {
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
    pub roles: Vec<ListPracticeTeamInvitationsResponseDataItemPersonAccountRolesItem>,
    #[serde(rename = "prescriberConnection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_connection:
        Option<ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection>,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonAccount {
    pub fn builder() -> ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder {
        <ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder {
    account_id: Option<String>,
    email_verified: Option<bool>,
    membership_id: Option<String>,
    membership_status: Option<String>,
    roles: Option<Vec<ListPracticeTeamInvitationsResponseDataItemPersonAccountRolesItem>>,
    prescriber_connection:
        Option<ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection>,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder {
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
        value: Vec<ListPracticeTeamInvitationsResponseDataItemPersonAccountRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn prescriber_connection(
        mut self,
        value: ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnection,
    ) -> Self {
        self.prescriber_connection = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsResponseDataItemPersonAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder::account_id)
    /// - [`email_verified`](ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder::email_verified)
    /// - [`membership_id`](ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder::membership_id)
    /// - [`membership_status`](ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder::membership_status)
    /// - [`roles`](ListPracticeTeamInvitationsResponseDataItemPersonAccountBuilder::roles)
    pub fn build(
        self,
    ) -> Result<ListPracticeTeamInvitationsResponseDataItemPersonAccount, BuildError> {
        Ok(ListPracticeTeamInvitationsResponseDataItemPersonAccount {
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

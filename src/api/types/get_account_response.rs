pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAccountResponse {
    pub account: GetAccountResponseAccount,
    /// True for a Live request; false for a Test request.
    #[serde(default)]
    pub livemode: bool,
    /// Effective scopes of the authenticated API key. Null for a dashboard session; use membership.permissions for that session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    pub membership: GetAccountResponseMembership,
    /// The organization's Live-access status, independent of this request's livemode.
    #[serde(rename = "operatingMode")]
    pub operating_mode: GetAccountResponseOperatingMode,
    #[serde(default)]
    pub user: GetAccountResponseUser,
}

impl GetAccountResponse {
    pub fn builder() -> GetAccountResponseBuilder {
        <GetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseBuilder {
    account: Option<GetAccountResponseAccount>,
    livemode: Option<bool>,
    scopes: Option<Vec<String>>,
    membership: Option<GetAccountResponseMembership>,
    operating_mode: Option<GetAccountResponseOperatingMode>,
    user: Option<GetAccountResponseUser>,
}

impl GetAccountResponseBuilder {
    pub fn account(mut self, value: GetAccountResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn membership(mut self, value: GetAccountResponseMembership) -> Self {
        self.membership = Some(value);
        self
    }

    pub fn operating_mode(mut self, value: GetAccountResponseOperatingMode) -> Self {
        self.operating_mode = Some(value);
        self
    }

    pub fn user(mut self, value: GetAccountResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](GetAccountResponseBuilder::account)
    /// - [`livemode`](GetAccountResponseBuilder::livemode)
    /// - [`membership`](GetAccountResponseBuilder::membership)
    /// - [`operating_mode`](GetAccountResponseBuilder::operating_mode)
    /// - [`user`](GetAccountResponseBuilder::user)
    pub fn build(self) -> Result<GetAccountResponse, BuildError> {
        Ok(GetAccountResponse {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            scopes: self.scopes,
            membership: self
                .membership
                .ok_or_else(|| BuildError::missing_field("membership"))?,
            operating_mode: self
                .operating_mode
                .ok_or_else(|| BuildError::missing_field("operating_mode"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}

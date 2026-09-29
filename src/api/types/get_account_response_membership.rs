pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetAccountResponseMembership {
    /// Effective API scopes for a service key; dashboard permissions for a signed-in member.
    #[serde(default)]
    pub permissions: Vec<String>,
    pub role: GetAccountResponseMembershipRole,
    #[serde(rename = "roleName")]
    #[serde(default)]
    pub role_name: String,
    pub status: GetAccountResponseMembershipStatus,
}

impl GetAccountResponseMembership {
    pub fn builder() -> GetAccountResponseMembershipBuilder {
        <GetAccountResponseMembershipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseMembershipBuilder {
    permissions: Option<Vec<String>>,
    role: Option<GetAccountResponseMembershipRole>,
    role_name: Option<String>,
    status: Option<GetAccountResponseMembershipStatus>,
}

impl GetAccountResponseMembershipBuilder {
    pub fn permissions(mut self, value: Vec<String>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn role(mut self, value: GetAccountResponseMembershipRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn role_name(mut self, value: impl Into<String>) -> Self {
        self.role_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetAccountResponseMembershipStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseMembership`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](GetAccountResponseMembershipBuilder::permissions)
    /// - [`role`](GetAccountResponseMembershipBuilder::role)
    /// - [`role_name`](GetAccountResponseMembershipBuilder::role_name)
    /// - [`status`](GetAccountResponseMembershipBuilder::status)
    pub fn build(self) -> Result<GetAccountResponseMembership, BuildError> {
        Ok(GetAccountResponseMembership {
            permissions: self
                .permissions
                .ok_or_else(|| BuildError::missing_field("permissions"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            role_name: self
                .role_name
                .ok_or_else(|| BuildError::missing_field("role_name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

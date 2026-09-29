pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountResponseUser {
    #[serde(default)]
    pub email: String,
    #[serde(rename = "emailVerified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "twoFactorEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_enabled: Option<bool>,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

impl GetAccountResponseUser {
    pub fn builder() -> GetAccountResponseUserBuilder {
        <GetAccountResponseUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountResponseUserBuilder {
    email: Option<String>,
    email_verified: Option<bool>,
    image: Option<String>,
    name: Option<String>,
    two_factor_enabled: Option<bool>,
    user_id: Option<String>,
}

impl GetAccountResponseUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn email_verified(mut self, value: bool) -> Self {
        self.email_verified = Some(value);
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn two_factor_enabled(mut self, value: bool) -> Self {
        self.two_factor_enabled = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountResponseUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetAccountResponseUserBuilder::email)
    /// - [`name`](GetAccountResponseUserBuilder::name)
    /// - [`user_id`](GetAccountResponseUserBuilder::user_id)
    pub fn build(self) -> Result<GetAccountResponseUser, BuildError> {
        Ok(GetAccountResponseUser {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            email_verified: self.email_verified,
            image: self.image,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            two_factor_enabled: self.two_factor_enabled,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}

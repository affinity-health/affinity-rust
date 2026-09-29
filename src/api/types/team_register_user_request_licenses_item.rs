pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestLicensesItem {
    #[serde(default)]
    pub state: String,
    #[serde(rename = "licenseNumber")]
    #[serde(default)]
    pub license_number: String,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl RegisterUserRequestLicensesItem {
    pub fn builder() -> RegisterUserRequestLicensesItemBuilder {
        <RegisterUserRequestLicensesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestLicensesItemBuilder {
    state: Option<String>,
    license_number: Option<String>,
    expires_at: Option<String>,
}

impl RegisterUserRequestLicensesItemBuilder {
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn license_number(mut self, value: impl Into<String>) -> Self {
        self.license_number = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequestLicensesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](RegisterUserRequestLicensesItemBuilder::state)
    /// - [`license_number`](RegisterUserRequestLicensesItemBuilder::license_number)
    pub fn build(self) -> Result<RegisterUserRequestLicensesItem, BuildError> {
        Ok(RegisterUserRequestLicensesItem {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            license_number: self
                .license_number
                .ok_or_else(|| BuildError::missing_field("license_number"))?,
            expires_at: self.expires_at,
        })
    }
}

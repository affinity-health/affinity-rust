pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestProfileDetailsCertificationsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    pub expires_at: String,
}

impl RegisterUserRequestProfileDetailsCertificationsItem {
    pub fn builder() -> RegisterUserRequestProfileDetailsCertificationsItemBuilder {
        <RegisterUserRequestProfileDetailsCertificationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestProfileDetailsCertificationsItemBuilder {
    name: Option<String>,
    issuer: Option<String>,
    expires_at: Option<String>,
}

impl RegisterUserRequestProfileDetailsCertificationsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequestProfileDetailsCertificationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RegisterUserRequestProfileDetailsCertificationsItemBuilder::name)
    /// - [`issuer`](RegisterUserRequestProfileDetailsCertificationsItemBuilder::issuer)
    /// - [`expires_at`](RegisterUserRequestProfileDetailsCertificationsItemBuilder::expires_at)
    pub fn build(self) -> Result<RegisterUserRequestProfileDetailsCertificationsItem, BuildError> {
        Ok(RegisterUserRequestProfileDetailsCertificationsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}

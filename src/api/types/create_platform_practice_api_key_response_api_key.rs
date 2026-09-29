pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePlatformPracticeApiKeyResponseApiKey {
    #[serde(rename = "allowedIps")]
    #[serde(default)]
    pub allowed_ips: Vec<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    pub key_prefix: String,
    #[serde(rename = "lastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    pub mode: CreatePlatformPracticeApiKeyResponseApiKeyMode,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<String>,
    #[serde(default)]
    pub scopes: Vec<CreatePlatformPracticeApiKeyResponseApiKeyScopesItem>,
    pub status: CreatePlatformPracticeApiKeyResponseApiKeyStatus,
}

impl CreatePlatformPracticeApiKeyResponseApiKey {
    pub fn builder() -> CreatePlatformPracticeApiKeyResponseApiKeyBuilder {
        <CreatePlatformPracticeApiKeyResponseApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformPracticeApiKeyResponseApiKeyBuilder {
    allowed_ips: Option<Vec<String>>,
    created_at: Option<String>,
    expires_at: Option<String>,
    id: Option<String>,
    key_prefix: Option<String>,
    last_used_at: Option<String>,
    mode: Option<CreatePlatformPracticeApiKeyResponseApiKeyMode>,
    name: Option<String>,
    revoked_at: Option<String>,
    scopes: Option<Vec<CreatePlatformPracticeApiKeyResponseApiKeyScopesItem>>,
    status: Option<CreatePlatformPracticeApiKeyResponseApiKeyStatus>,
}

impl CreatePlatformPracticeApiKeyResponseApiKeyBuilder {
    pub fn allowed_ips(mut self, value: Vec<String>) -> Self {
        self.allowed_ips = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn key_prefix(mut self, value: impl Into<String>) -> Self {
        self.key_prefix = Some(value.into());
        self
    }

    pub fn last_used_at(mut self, value: impl Into<String>) -> Self {
        self.last_used_at = Some(value.into());
        self
    }

    pub fn mode(mut self, value: CreatePlatformPracticeApiKeyResponseApiKeyMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: impl Into<String>) -> Self {
        self.revoked_at = Some(value.into());
        self
    }

    pub fn scopes(
        mut self,
        value: Vec<CreatePlatformPracticeApiKeyResponseApiKeyScopesItem>,
    ) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn status(mut self, value: CreatePlatformPracticeApiKeyResponseApiKeyStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformPracticeApiKeyResponseApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed_ips`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::allowed_ips)
    /// - [`created_at`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::created_at)
    /// - [`id`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::id)
    /// - [`key_prefix`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::key_prefix)
    /// - [`mode`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::mode)
    /// - [`name`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::name)
    /// - [`scopes`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::scopes)
    /// - [`status`](CreatePlatformPracticeApiKeyResponseApiKeyBuilder::status)
    pub fn build(self) -> Result<CreatePlatformPracticeApiKeyResponseApiKey, BuildError> {
        Ok(CreatePlatformPracticeApiKeyResponseApiKey {
            allowed_ips: self
                .allowed_ips
                .ok_or_else(|| BuildError::missing_field("allowed_ips"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            key_prefix: self
                .key_prefix
                .ok_or_else(|| BuildError::missing_field("key_prefix"))?,
            last_used_at: self.last_used_at,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            revoked_at: self.revoked_at,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

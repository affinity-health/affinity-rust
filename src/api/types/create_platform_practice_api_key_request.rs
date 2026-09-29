pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlatformPracticeApiKeyRequest {
    #[serde(rename = "allowedIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<CreatePlatformPracticeApiKeyRequestScopesItem>>,
}

impl CreatePlatformPracticeApiKeyRequest {
    pub fn builder() -> CreatePlatformPracticeApiKeyRequestBuilder {
        <CreatePlatformPracticeApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformPracticeApiKeyRequestBuilder {
    allowed_ips: Option<Vec<String>>,
    expires_at: Option<String>,
    name: Option<String>,
    scopes: Option<Vec<CreatePlatformPracticeApiKeyRequestScopesItem>>,
}

impl CreatePlatformPracticeApiKeyRequestBuilder {
    pub fn allowed_ips(mut self, value: Vec<String>) -> Self {
        self.allowed_ips = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<CreatePlatformPracticeApiKeyRequestScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformPracticeApiKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreatePlatformPracticeApiKeyRequestBuilder::name)
    pub fn build(self) -> Result<CreatePlatformPracticeApiKeyRequest, BuildError> {
        Ok(CreatePlatformPracticeApiKeyRequest {
            allowed_ips: self.allowed_ips,
            expires_at: self.expires_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scopes: self.scopes,
        })
    }
}

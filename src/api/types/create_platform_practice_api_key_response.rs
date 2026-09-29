pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePlatformPracticeApiKeyResponse {
    #[serde(rename = "apiKey")]
    pub api_key: CreatePlatformPracticeApiKeyResponseApiKey,
    #[serde(default)]
    pub secret: String,
    #[serde(rename = "serviceAccount")]
    pub service_account: CreatePlatformPracticeApiKeyResponseServiceAccount,
}

impl CreatePlatformPracticeApiKeyResponse {
    pub fn builder() -> CreatePlatformPracticeApiKeyResponseBuilder {
        <CreatePlatformPracticeApiKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformPracticeApiKeyResponseBuilder {
    api_key: Option<CreatePlatformPracticeApiKeyResponseApiKey>,
    secret: Option<String>,
    service_account: Option<CreatePlatformPracticeApiKeyResponseServiceAccount>,
}

impl CreatePlatformPracticeApiKeyResponseBuilder {
    pub fn api_key(mut self, value: CreatePlatformPracticeApiKeyResponseApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn service_account(
        mut self,
        value: CreatePlatformPracticeApiKeyResponseServiceAccount,
    ) -> Self {
        self.service_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformPracticeApiKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](CreatePlatformPracticeApiKeyResponseBuilder::api_key)
    /// - [`secret`](CreatePlatformPracticeApiKeyResponseBuilder::secret)
    /// - [`service_account`](CreatePlatformPracticeApiKeyResponseBuilder::service_account)
    pub fn build(self) -> Result<CreatePlatformPracticeApiKeyResponse, BuildError> {
        Ok(CreatePlatformPracticeApiKeyResponse {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            secret: self
                .secret
                .ok_or_else(|| BuildError::missing_field("secret"))?,
            service_account: self
                .service_account
                .ok_or_else(|| BuildError::missing_field("service_account"))?,
        })
    }
}

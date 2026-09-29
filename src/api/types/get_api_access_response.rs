pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetApiAccessResponse {
    #[serde(rename = "apiKey")]
    pub api_key: GetApiAccessResponseApiKey,
    #[serde(default)]
    pub livemode: bool,
    pub object: GetApiAccessResponseObject,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(rename = "serviceAccount")]
    pub service_account: GetApiAccessResponseServiceAccount,
}

impl GetApiAccessResponse {
    pub fn builder() -> GetApiAccessResponseBuilder {
        <GetApiAccessResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetApiAccessResponseBuilder {
    api_key: Option<GetApiAccessResponseApiKey>,
    livemode: Option<bool>,
    object: Option<GetApiAccessResponseObject>,
    scopes: Option<Vec<String>>,
    service_account: Option<GetApiAccessResponseServiceAccount>,
}

impl GetApiAccessResponseBuilder {
    pub fn api_key(mut self, value: GetApiAccessResponseApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: GetApiAccessResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn service_account(mut self, value: GetApiAccessResponseServiceAccount) -> Self {
        self.service_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetApiAccessResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](GetApiAccessResponseBuilder::api_key)
    /// - [`livemode`](GetApiAccessResponseBuilder::livemode)
    /// - [`object`](GetApiAccessResponseBuilder::object)
    /// - [`scopes`](GetApiAccessResponseBuilder::scopes)
    /// - [`service_account`](GetApiAccessResponseBuilder::service_account)
    pub fn build(self) -> Result<GetApiAccessResponse, BuildError> {
        Ok(GetApiAccessResponse {
            api_key: self
                .api_key
                .ok_or_else(|| BuildError::missing_field("api_key"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            service_account: self
                .service_account
                .ok_or_else(|| BuildError::missing_field("service_account"))?,
        })
    }
}

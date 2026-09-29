pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RevokeWebhookGrantResponse {
    pub object: RevokeWebhookGrantResponseObject,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(default)]
    pub revoked: bool,
}

impl RevokeWebhookGrantResponse {
    pub fn builder() -> RevokeWebhookGrantResponseBuilder {
        <RevokeWebhookGrantResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWebhookGrantResponseBuilder {
    object: Option<RevokeWebhookGrantResponseObject>,
    organization_id: Option<String>,
    platform_id: Option<String>,
    revoked: Option<bool>,
}

impl RevokeWebhookGrantResponseBuilder {
    pub fn object(mut self, value: RevokeWebhookGrantResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn platform_id(mut self, value: impl Into<String>) -> Self {
        self.platform_id = Some(value.into());
        self
    }

    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevokeWebhookGrantResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](RevokeWebhookGrantResponseBuilder::object)
    /// - [`organization_id`](RevokeWebhookGrantResponseBuilder::organization_id)
    /// - [`platform_id`](RevokeWebhookGrantResponseBuilder::platform_id)
    /// - [`revoked`](RevokeWebhookGrantResponseBuilder::revoked)
    pub fn build(self) -> Result<RevokeWebhookGrantResponse, BuildError> {
        Ok(RevokeWebhookGrantResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            platform_id: self
                .platform_id
                .ok_or_else(|| BuildError::missing_field("platform_id"))?,
            revoked: self
                .revoked
                .ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}

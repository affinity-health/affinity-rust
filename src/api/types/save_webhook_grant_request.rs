pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveWebhookGrantRequest {
    #[serde(default)]
    pub scopes: Vec<SaveWebhookGrantRequestScopesItem>,
}

impl SaveWebhookGrantRequest {
    pub fn builder() -> SaveWebhookGrantRequestBuilder {
        <SaveWebhookGrantRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveWebhookGrantRequestBuilder {
    scopes: Option<Vec<SaveWebhookGrantRequestScopesItem>>,
}

impl SaveWebhookGrantRequestBuilder {
    pub fn scopes(mut self, value: Vec<SaveWebhookGrantRequestScopesItem>) -> Self {
        self.scopes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SaveWebhookGrantRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scopes`](SaveWebhookGrantRequestBuilder::scopes)
    pub fn build(self) -> Result<SaveWebhookGrantRequest, BuildError> {
        Ok(SaveWebhookGrantRequest {
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
        })
    }
}

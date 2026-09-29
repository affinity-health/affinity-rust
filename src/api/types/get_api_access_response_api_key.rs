pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetApiAccessResponseApiKey {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    pub key_prefix: String,
    pub object: GetApiAccessResponseApiKeyObject,
}

impl GetApiAccessResponseApiKey {
    pub fn builder() -> GetApiAccessResponseApiKeyBuilder {
        <GetApiAccessResponseApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetApiAccessResponseApiKeyBuilder {
    id: Option<String>,
    key_prefix: Option<String>,
    object: Option<GetApiAccessResponseApiKeyObject>,
}

impl GetApiAccessResponseApiKeyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn key_prefix(mut self, value: impl Into<String>) -> Self {
        self.key_prefix = Some(value.into());
        self
    }

    pub fn object(mut self, value: GetApiAccessResponseApiKeyObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetApiAccessResponseApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetApiAccessResponseApiKeyBuilder::id)
    /// - [`key_prefix`](GetApiAccessResponseApiKeyBuilder::key_prefix)
    /// - [`object`](GetApiAccessResponseApiKeyBuilder::object)
    pub fn build(self) -> Result<GetApiAccessResponseApiKey, BuildError> {
        Ok(GetApiAccessResponseApiKey {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            key_prefix: self
                .key_prefix
                .ok_or_else(|| BuildError::missing_field("key_prefix"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}

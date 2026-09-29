pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SaveWebhookGrantRequestScopesItem {
    WebhooksRead,
    WebhooksWrite,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SaveWebhookGrantRequestScopesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WebhooksRead => serializer.serialize_str("webhooks:read"),
            Self::WebhooksWrite => serializer.serialize_str("webhooks:write"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SaveWebhookGrantRequestScopesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "webhooks:read" => Ok(Self::WebhooksRead),
            "webhooks:write" => Ok(Self::WebhooksWrite),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SaveWebhookGrantRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebhooksRead => write!(f, "webhooks:read"),
            Self::WebhooksWrite => write!(f, "webhooks:write"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

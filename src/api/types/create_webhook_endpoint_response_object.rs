pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateWebhookEndpointResponseObject {
    #[serde(rename = "webhook_endpoint")]
    WebhookEndpoint,
}
impl fmt::Display for CreateWebhookEndpointResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WebhookEndpoint => "webhook_endpoint",
        };
        write!(f, "{}", s)
    }
}

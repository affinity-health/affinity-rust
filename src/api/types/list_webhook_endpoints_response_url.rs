pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListWebhookEndpointsResponseUrl {
    #[serde(rename = "/v1/webhook-endpoints")]
    V1WebhookEndpoints,
}
impl fmt::Display for ListWebhookEndpointsResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1WebhookEndpoints => "/v1/webhook-endpoints",
        };
        write!(f, "{}", s)
    }
}

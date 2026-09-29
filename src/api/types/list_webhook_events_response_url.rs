pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListWebhookEventsResponseUrl {
    #[serde(rename = "/v1/webhook-events")]
    V1WebhookEvents,
}
impl fmt::Display for ListWebhookEventsResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1WebhookEvents => "/v1/webhook-events",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetWebhookEventResponseObject {
    #[serde(rename = "webhook_event")]
    WebhookEvent,
}
impl fmt::Display for GetWebhookEventResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WebhookEvent => "webhook_event",
        };
        write!(f, "{}", s)
    }
}

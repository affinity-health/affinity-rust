pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListWebhookEventsResponseDataItemObject {
    #[serde(rename = "webhook_event")]
    WebhookEvent,
}
impl fmt::Display for ListWebhookEventsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WebhookEvent => "webhook_event",
        };
        write!(f, "{}", s)
    }
}

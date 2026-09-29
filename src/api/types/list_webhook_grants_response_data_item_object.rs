pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListWebhookGrantsResponseDataItemObject {
    #[serde(rename = "webhook_grant")]
    WebhookGrant,
}
impl fmt::Display for ListWebhookGrantsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WebhookGrant => "webhook_grant",
        };
        write!(f, "{}", s)
    }
}

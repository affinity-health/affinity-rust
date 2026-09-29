pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListWebhookGrantsResponseUrl {
    #[serde(rename = "/v1/webhook-grants")]
    V1WebhookGrants,
}
impl fmt::Display for ListWebhookGrantsResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1WebhookGrants => "/v1/webhook-grants",
        };
        write!(f, "{}", s)
    }
}

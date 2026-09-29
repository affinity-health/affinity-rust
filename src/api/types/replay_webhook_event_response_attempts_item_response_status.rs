pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplayWebhookEventResponseAttemptsItemResponseStatus {
    Double(f64),

    ReplayWebhookEventResponseAttemptsItemResponseStatusOne(
        ReplayWebhookEventResponseAttemptsItemResponseStatusOne,
    ),
}

impl ReplayWebhookEventResponseAttemptsItemResponseStatus {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_replay_webhook_event_response_attempts_item_response_status_one(&self) -> bool {
        matches!(
            self,
            Self::ReplayWebhookEventResponseAttemptsItemResponseStatusOne(_)
        )
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_replay_webhook_event_response_attempts_item_response_status_one(
        &self,
    ) -> Option<&ReplayWebhookEventResponseAttemptsItemResponseStatusOne> {
        match self {
            Self::ReplayWebhookEventResponseAttemptsItemResponseStatusOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_replay_webhook_event_response_attempts_item_response_status_one(
        self,
    ) -> Option<ReplayWebhookEventResponseAttemptsItemResponseStatusOne> {
        match self {
            Self::ReplayWebhookEventResponseAttemptsItemResponseStatusOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReplayWebhookEventResponseAttemptsItemResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ReplayWebhookEventResponseAttemptsItemResponseStatusOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

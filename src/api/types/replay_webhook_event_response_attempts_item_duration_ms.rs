pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplayWebhookEventResponseAttemptsItemDurationMs {
    Double(f64),

    ReplayWebhookEventResponseAttemptsItemDurationMsOne(
        ReplayWebhookEventResponseAttemptsItemDurationMsOne,
    ),
}

impl ReplayWebhookEventResponseAttemptsItemDurationMs {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_replay_webhook_event_response_attempts_item_duration_ms_one(&self) -> bool {
        matches!(
            self,
            Self::ReplayWebhookEventResponseAttemptsItemDurationMsOne(_)
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

    pub fn as_replay_webhook_event_response_attempts_item_duration_ms_one(
        &self,
    ) -> Option<&ReplayWebhookEventResponseAttemptsItemDurationMsOne> {
        match self {
            Self::ReplayWebhookEventResponseAttemptsItemDurationMsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_replay_webhook_event_response_attempts_item_duration_ms_one(
        self,
    ) -> Option<ReplayWebhookEventResponseAttemptsItemDurationMsOne> {
        match self {
            Self::ReplayWebhookEventResponseAttemptsItemDurationMsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ReplayWebhookEventResponseAttemptsItemDurationMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ReplayWebhookEventResponseAttemptsItemDurationMsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

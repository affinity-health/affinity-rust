pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    Double(f64),

    ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(
        ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne,
    ),
}

impl ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_replay_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(_)
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

    pub fn as_replay_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        &self,
    ) -> Option<&ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne> {
        match self {
            Self::ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_replay_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        self,
    ) -> Option<ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne> {
        match self {
            Self::ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ReplayWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

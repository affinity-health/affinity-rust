pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    Double(f64),

    GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(
        GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne,
    ),
}

impl GetWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(_)
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

    pub fn as_get_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        &self,
    ) -> Option<&GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne> {
        match self {
            Self::GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_get_webhook_event_response_deliveries_item_automatic_attempt_count_one(
        self,
    ) -> Option<GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne> {
        match self {
            Self::GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for GetWebhookEventResponseDeliveriesItemAutomaticAttemptCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetWebhookEventResponseDeliveriesItemAutomaticAttemptCountOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

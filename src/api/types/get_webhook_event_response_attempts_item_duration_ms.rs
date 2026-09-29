pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetWebhookEventResponseAttemptsItemDurationMs {
    Double(f64),

    GetWebhookEventResponseAttemptsItemDurationMsOne(
        GetWebhookEventResponseAttemptsItemDurationMsOne,
    ),
}

impl GetWebhookEventResponseAttemptsItemDurationMs {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_webhook_event_response_attempts_item_duration_ms_one(&self) -> bool {
        matches!(
            self,
            Self::GetWebhookEventResponseAttemptsItemDurationMsOne(_)
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

    pub fn as_get_webhook_event_response_attempts_item_duration_ms_one(
        &self,
    ) -> Option<&GetWebhookEventResponseAttemptsItemDurationMsOne> {
        match self {
            Self::GetWebhookEventResponseAttemptsItemDurationMsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_webhook_event_response_attempts_item_duration_ms_one(
        self,
    ) -> Option<GetWebhookEventResponseAttemptsItemDurationMsOne> {
        match self {
            Self::GetWebhookEventResponseAttemptsItemDurationMsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetWebhookEventResponseAttemptsItemDurationMs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetWebhookEventResponseAttemptsItemDurationMsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

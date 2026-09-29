pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetWebhookEventResponseAttemptsItemResponseStatus {
    Double(f64),

    GetWebhookEventResponseAttemptsItemResponseStatusOne(
        GetWebhookEventResponseAttemptsItemResponseStatusOne,
    ),
}

impl GetWebhookEventResponseAttemptsItemResponseStatus {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_webhook_event_response_attempts_item_response_status_one(&self) -> bool {
        matches!(
            self,
            Self::GetWebhookEventResponseAttemptsItemResponseStatusOne(_)
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

    pub fn as_get_webhook_event_response_attempts_item_response_status_one(
        &self,
    ) -> Option<&GetWebhookEventResponseAttemptsItemResponseStatusOne> {
        match self {
            Self::GetWebhookEventResponseAttemptsItemResponseStatusOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_webhook_event_response_attempts_item_response_status_one(
        self,
    ) -> Option<GetWebhookEventResponseAttemptsItemResponseStatusOne> {
        match self {
            Self::GetWebhookEventResponseAttemptsItemResponseStatusOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetWebhookEventResponseAttemptsItemResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetWebhookEventResponseAttemptsItemResponseStatusOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SignAndSubmitOrderResponsePrescriptionsItemErrorStatus {
    Double(f64),

    SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne(
        SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne,
    ),
}

impl SignAndSubmitOrderResponsePrescriptionsItemErrorStatus {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_sign_and_submit_order_response_prescriptions_item_error_status_one(&self) -> bool {
        matches!(
            self,
            Self::SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne(_)
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

    pub fn as_sign_and_submit_order_response_prescriptions_item_error_status_one(
        &self,
    ) -> Option<&SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne> {
        match self {
            Self::SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_sign_and_submit_order_response_prescriptions_item_error_status_one(
        self,
    ) -> Option<SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne> {
        match self {
            Self::SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for SignAndSubmitOrderResponsePrescriptionsItemErrorStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

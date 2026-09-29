pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetOrderResponsePrescriptionsItemQuantity {
    Double(f64),

    GetOrderResponsePrescriptionsItemQuantityOne(GetOrderResponsePrescriptionsItemQuantityOne),
}

impl GetOrderResponsePrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_order_response_prescriptions_item_quantity_one(&self) -> bool {
        matches!(self, Self::GetOrderResponsePrescriptionsItemQuantityOne(_))
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

    pub fn as_get_order_response_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&GetOrderResponsePrescriptionsItemQuantityOne> {
        match self {
            Self::GetOrderResponsePrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_order_response_prescriptions_item_quantity_one(
        self,
    ) -> Option<GetOrderResponsePrescriptionsItemQuantityOne> {
        match self {
            Self::GetOrderResponsePrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetOrderResponsePrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetOrderResponsePrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

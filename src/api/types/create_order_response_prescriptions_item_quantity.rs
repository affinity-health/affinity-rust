pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderResponsePrescriptionsItemQuantity {
    Double(f64),

    CreateOrderResponsePrescriptionsItemQuantityOne(
        CreateOrderResponsePrescriptionsItemQuantityOne,
    ),
}

impl CreateOrderResponsePrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_response_prescriptions_item_quantity_one(&self) -> bool {
        matches!(
            self,
            Self::CreateOrderResponsePrescriptionsItemQuantityOne(_)
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

    pub fn as_create_order_response_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&CreateOrderResponsePrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderResponsePrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_response_prescriptions_item_quantity_one(
        self,
    ) -> Option<CreateOrderResponsePrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderResponsePrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderResponsePrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderResponsePrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

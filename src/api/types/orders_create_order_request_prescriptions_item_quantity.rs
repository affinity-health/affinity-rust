pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderRequestPrescriptionsItemQuantity {
    Double(f64),

    CreateOrderRequestPrescriptionsItemQuantityOne(CreateOrderRequestPrescriptionsItemQuantityOne),
}

impl CreateOrderRequestPrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_request_prescriptions_item_quantity_one(&self) -> bool {
        matches!(
            self,
            Self::CreateOrderRequestPrescriptionsItemQuantityOne(_)
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

    pub fn as_create_order_request_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&CreateOrderRequestPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderRequestPrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_request_prescriptions_item_quantity_one(
        self,
    ) -> Option<CreateOrderRequestPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderRequestPrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderRequestPrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderRequestPrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

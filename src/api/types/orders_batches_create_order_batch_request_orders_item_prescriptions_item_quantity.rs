pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity {
    Double(f64),

    CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne(
        CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne,
    ),
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_request_orders_item_prescriptions_item_quantity_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne(_)
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

    pub fn as_create_order_batch_request_orders_item_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_create_order_batch_request_orders_item_prescriptions_item_quantity_one(
        self,
    ) -> Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

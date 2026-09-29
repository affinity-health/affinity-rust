pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantity {
    Double(f64),

    CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne(
        CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne,
    ),
}

impl CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_response_orders_item_prescriptions_item_quantity_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne(_)
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

    pub fn as_create_order_batch_response_orders_item_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_create_order_batch_response_orders_item_prescriptions_item_quantity_one(
        self,
    ) -> Option<CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne> {
        match self {
            Self::CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchResponseOrdersItemPrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

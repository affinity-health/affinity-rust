pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListOrdersResponseDataItemPrescriptionsItemQuantity {
    Double(f64),

    ListOrdersResponseDataItemPrescriptionsItemQuantityOne(
        ListOrdersResponseDataItemPrescriptionsItemQuantityOne,
    ),
}

impl ListOrdersResponseDataItemPrescriptionsItemQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_orders_response_data_item_prescriptions_item_quantity_one(&self) -> bool {
        matches!(
            self,
            Self::ListOrdersResponseDataItemPrescriptionsItemQuantityOne(_)
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

    pub fn as_list_orders_response_data_item_prescriptions_item_quantity_one(
        &self,
    ) -> Option<&ListOrdersResponseDataItemPrescriptionsItemQuantityOne> {
        match self {
            Self::ListOrdersResponseDataItemPrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_orders_response_data_item_prescriptions_item_quantity_one(
        self,
    ) -> Option<ListOrdersResponseDataItemPrescriptionsItemQuantityOne> {
        match self {
            Self::ListOrdersResponseDataItemPrescriptionsItemQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ListOrdersResponseDataItemPrescriptionsItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListOrdersResponseDataItemPrescriptionsItemQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

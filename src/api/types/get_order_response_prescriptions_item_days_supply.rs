pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetOrderResponsePrescriptionsItemDaysSupply {
    Double(f64),

    GetOrderResponsePrescriptionsItemDaysSupplyOne(GetOrderResponsePrescriptionsItemDaysSupplyOne),
}

impl GetOrderResponsePrescriptionsItemDaysSupply {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_order_response_prescriptions_item_days_supply_one(&self) -> bool {
        matches!(
            self,
            Self::GetOrderResponsePrescriptionsItemDaysSupplyOne(_)
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

    pub fn as_get_order_response_prescriptions_item_days_supply_one(
        &self,
    ) -> Option<&GetOrderResponsePrescriptionsItemDaysSupplyOne> {
        match self {
            Self::GetOrderResponsePrescriptionsItemDaysSupplyOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_order_response_prescriptions_item_days_supply_one(
        self,
    ) -> Option<GetOrderResponsePrescriptionsItemDaysSupplyOne> {
        match self {
            Self::GetOrderResponsePrescriptionsItemDaysSupplyOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetOrderResponsePrescriptionsItemDaysSupply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetOrderResponsePrescriptionsItemDaysSupplyOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

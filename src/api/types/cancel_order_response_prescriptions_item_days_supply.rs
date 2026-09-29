pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CancelOrderResponsePrescriptionsItemDaysSupply {
    Double(f64),

    CancelOrderResponsePrescriptionsItemDaysSupplyOne(
        CancelOrderResponsePrescriptionsItemDaysSupplyOne,
    ),
}

impl CancelOrderResponsePrescriptionsItemDaysSupply {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_cancel_order_response_prescriptions_item_days_supply_one(&self) -> bool {
        matches!(
            self,
            Self::CancelOrderResponsePrescriptionsItemDaysSupplyOne(_)
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

    pub fn as_cancel_order_response_prescriptions_item_days_supply_one(
        &self,
    ) -> Option<&CancelOrderResponsePrescriptionsItemDaysSupplyOne> {
        match self {
            Self::CancelOrderResponsePrescriptionsItemDaysSupplyOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_cancel_order_response_prescriptions_item_days_supply_one(
        self,
    ) -> Option<CancelOrderResponsePrescriptionsItemDaysSupplyOne> {
        match self {
            Self::CancelOrderResponsePrescriptionsItemDaysSupplyOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CancelOrderResponsePrescriptionsItemDaysSupply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CancelOrderResponsePrescriptionsItemDaysSupplyOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

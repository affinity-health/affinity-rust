pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdateOrderPrescriptionRequestPrescriptionQuantity {
    Double(f64),

    UpdateOrderPrescriptionRequestPrescriptionQuantityOne(
        UpdateOrderPrescriptionRequestPrescriptionQuantityOne,
    ),
}

impl UpdateOrderPrescriptionRequestPrescriptionQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_order_prescription_request_prescription_quantity_one(&self) -> bool {
        matches!(
            self,
            Self::UpdateOrderPrescriptionRequestPrescriptionQuantityOne(_)
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

    pub fn as_update_order_prescription_request_prescription_quantity_one(
        &self,
    ) -> Option<&UpdateOrderPrescriptionRequestPrescriptionQuantityOne> {
        match self {
            Self::UpdateOrderPrescriptionRequestPrescriptionQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_order_prescription_request_prescription_quantity_one(
        self,
    ) -> Option<UpdateOrderPrescriptionRequestPrescriptionQuantityOne> {
        match self {
            Self::UpdateOrderPrescriptionRequestPrescriptionQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdateOrderPrescriptionRequestPrescriptionQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdateOrderPrescriptionRequestPrescriptionQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

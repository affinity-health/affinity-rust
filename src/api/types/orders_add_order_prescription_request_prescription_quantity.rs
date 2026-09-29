pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AddOrderPrescriptionRequestPrescriptionQuantity {
    Double(f64),

    AddOrderPrescriptionRequestPrescriptionQuantityOne(
        AddOrderPrescriptionRequestPrescriptionQuantityOne,
    ),
}

impl AddOrderPrescriptionRequestPrescriptionQuantity {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_add_order_prescription_request_prescription_quantity_one(&self) -> bool {
        matches!(
            self,
            Self::AddOrderPrescriptionRequestPrescriptionQuantityOne(_)
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

    pub fn as_add_order_prescription_request_prescription_quantity_one(
        &self,
    ) -> Option<&AddOrderPrescriptionRequestPrescriptionQuantityOne> {
        match self {
            Self::AddOrderPrescriptionRequestPrescriptionQuantityOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_add_order_prescription_request_prescription_quantity_one(
        self,
    ) -> Option<AddOrderPrescriptionRequestPrescriptionQuantityOne> {
        match self {
            Self::AddOrderPrescriptionRequestPrescriptionQuantityOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AddOrderPrescriptionRequestPrescriptionQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::AddOrderPrescriptionRequestPrescriptionQuantityOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

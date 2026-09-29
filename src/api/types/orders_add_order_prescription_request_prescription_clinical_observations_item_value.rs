pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    Double(f64),

    AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
        AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne,
    ),
}

impl AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_add_order_prescription_request_prescription_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(_)
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

    pub fn as_add_order_prescription_request_prescription_clinical_observations_item_value_one(
        &self,
    ) -> Option<&AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne> {
        match self {
            Self::AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_add_order_prescription_request_prescription_clinical_observations_item_value_one(
        self,
    ) -> Option<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne> {
        match self {
            Self::AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

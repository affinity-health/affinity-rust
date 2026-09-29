pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    Double(f64),

    UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
        UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne,
    ),
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_order_prescription_request_prescription_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(_)
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

    pub fn as_update_order_prescription_request_prescription_clinical_observations_item_value_one(
        &self,
    ) -> Option<&UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne> {
        match self {
            Self::UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_order_prescription_request_prescription_clinical_observations_item_value_one(
        self,
    ) -> Option<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne> {
        match self {
            Self::UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValueOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

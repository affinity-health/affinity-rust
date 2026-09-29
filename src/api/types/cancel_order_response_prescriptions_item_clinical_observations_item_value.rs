pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue {
    Double(f64),

    CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne(
        CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne,
    ),
}

impl CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_cancel_order_response_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne(_)
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

    pub fn as_cancel_order_response_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> Option<&CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_cancel_order_response_prescriptions_item_clinical_observations_item_value_one(
        self,
    ) -> Option<CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CancelOrderResponsePrescriptionsItemClinicalObservationsItemValueOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

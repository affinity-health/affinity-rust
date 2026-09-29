pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue {
    Double(f64),

    CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne(
        CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne,
    ),
}

impl CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_request_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne(_)
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

    pub fn as_create_order_request_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> Option<&CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_create_order_request_prescriptions_item_clinical_observations_item_value_one(
        self,
    ) -> Option<CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderRequestPrescriptionsItemClinicalObservationsItemValueOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

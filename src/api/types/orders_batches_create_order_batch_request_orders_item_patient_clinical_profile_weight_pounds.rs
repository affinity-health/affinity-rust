pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPounds {
    Double(f64),

    CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne(
        CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne,
    ),
}

impl CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_request_orders_item_patient_clinical_profile_weight_pounds_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne(_)
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

    pub fn as_create_order_batch_request_orders_item_patient_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_create_order_batch_request_orders_item_patient_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchRequestOrdersItemPatientClinicalProfileWeightPoundsOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

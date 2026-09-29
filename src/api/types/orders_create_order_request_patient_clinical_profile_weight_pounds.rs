pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderRequestPatientClinicalProfileWeightPounds {
    Double(f64),

    CreateOrderRequestPatientClinicalProfileWeightPoundsOne(
        CreateOrderRequestPatientClinicalProfileWeightPoundsOne,
    ),
}

impl CreateOrderRequestPatientClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_request_patient_clinical_profile_weight_pounds_one(&self) -> bool {
        matches!(
            self,
            Self::CreateOrderRequestPatientClinicalProfileWeightPoundsOne(_)
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

    pub fn as_create_order_request_patient_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&CreateOrderRequestPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreateOrderRequestPatientClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_request_patient_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<CreateOrderRequestPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreateOrderRequestPatientClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderRequestPatientClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderRequestPatientClinicalProfileWeightPoundsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

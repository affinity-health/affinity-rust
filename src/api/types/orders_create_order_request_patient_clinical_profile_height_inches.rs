pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderRequestPatientClinicalProfileHeightInches {
    Double(f64),

    CreateOrderRequestPatientClinicalProfileHeightInchesOne(
        CreateOrderRequestPatientClinicalProfileHeightInchesOne,
    ),
}

impl CreateOrderRequestPatientClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_request_patient_clinical_profile_height_inches_one(&self) -> bool {
        matches!(
            self,
            Self::CreateOrderRequestPatientClinicalProfileHeightInchesOne(_)
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

    pub fn as_create_order_request_patient_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&CreateOrderRequestPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::CreateOrderRequestPatientClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_request_patient_clinical_profile_height_inches_one(
        self,
    ) -> Option<CreateOrderRequestPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::CreateOrderRequestPatientClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderRequestPatientClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderRequestPatientClinicalProfileHeightInchesOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

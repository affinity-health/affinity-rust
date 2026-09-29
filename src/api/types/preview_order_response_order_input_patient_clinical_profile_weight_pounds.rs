pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderResponseOrderInputPatientClinicalProfileWeightPounds {
    Double(f64),

    PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne(
        PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne,
    ),
}

impl PreviewOrderResponseOrderInputPatientClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_response_order_input_patient_clinical_profile_weight_pounds_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne(_)
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

    pub fn as_preview_order_response_order_input_patient_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_preview_order_response_order_input_patient_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderResponseOrderInputPatientClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileWeightPoundsOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

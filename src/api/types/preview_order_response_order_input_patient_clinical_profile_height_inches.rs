pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderResponseOrderInputPatientClinicalProfileHeightInches {
    Double(f64),

    PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne(
        PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne,
    ),
}

impl PreviewOrderResponseOrderInputPatientClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_response_order_input_patient_clinical_profile_height_inches_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne(_)
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

    pub fn as_preview_order_response_order_input_patient_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_preview_order_response_order_input_patient_clinical_profile_height_inches_one(
        self,
    ) -> Option<PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderResponseOrderInputPatientClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderResponseOrderInputPatientClinicalProfileHeightInchesOne(value) => {
                write!(
                    f,
                    "{}",
                    serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
                )
            }
        }
    }
}

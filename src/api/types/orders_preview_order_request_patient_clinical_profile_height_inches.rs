pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderRequestPatientClinicalProfileHeightInches {
    Double(f64),

    PreviewOrderRequestPatientClinicalProfileHeightInchesOne(
        PreviewOrderRequestPatientClinicalProfileHeightInchesOne,
    ),
}

impl PreviewOrderRequestPatientClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_request_patient_clinical_profile_height_inches_one(&self) -> bool {
        matches!(
            self,
            Self::PreviewOrderRequestPatientClinicalProfileHeightInchesOne(_)
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

    pub fn as_preview_order_request_patient_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&PreviewOrderRequestPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::PreviewOrderRequestPatientClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_preview_order_request_patient_clinical_profile_height_inches_one(
        self,
    ) -> Option<PreviewOrderRequestPatientClinicalProfileHeightInchesOne> {
        match self {
            Self::PreviewOrderRequestPatientClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderRequestPatientClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderRequestPatientClinicalProfileHeightInchesOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

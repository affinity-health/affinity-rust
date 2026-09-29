pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdatePatientRequestClinicalProfileHeightInches {
    Double(f64),

    UpdatePatientRequestClinicalProfileHeightInchesOne(
        UpdatePatientRequestClinicalProfileHeightInchesOne,
    ),
}

impl UpdatePatientRequestClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_patient_request_clinical_profile_height_inches_one(&self) -> bool {
        matches!(
            self,
            Self::UpdatePatientRequestClinicalProfileHeightInchesOne(_)
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

    pub fn as_update_patient_request_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&UpdatePatientRequestClinicalProfileHeightInchesOne> {
        match self {
            Self::UpdatePatientRequestClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_patient_request_clinical_profile_height_inches_one(
        self,
    ) -> Option<UpdatePatientRequestClinicalProfileHeightInchesOne> {
        match self {
            Self::UpdatePatientRequestClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdatePatientRequestClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdatePatientRequestClinicalProfileHeightInchesOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

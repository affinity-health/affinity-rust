pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientResponseClinicalProfileHeightInches {
    Double(f64),

    CreatePatientResponseClinicalProfileHeightInchesOne(
        CreatePatientResponseClinicalProfileHeightInchesOne,
    ),
}

impl CreatePatientResponseClinicalProfileHeightInches {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_response_clinical_profile_height_inches_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientResponseClinicalProfileHeightInchesOne(_)
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

    pub fn as_create_patient_response_clinical_profile_height_inches_one(
        &self,
    ) -> Option<&CreatePatientResponseClinicalProfileHeightInchesOne> {
        match self {
            Self::CreatePatientResponseClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_response_clinical_profile_height_inches_one(
        self,
    ) -> Option<CreatePatientResponseClinicalProfileHeightInchesOne> {
        match self {
            Self::CreatePatientResponseClinicalProfileHeightInchesOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientResponseClinicalProfileHeightInches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientResponseClinicalProfileHeightInchesOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

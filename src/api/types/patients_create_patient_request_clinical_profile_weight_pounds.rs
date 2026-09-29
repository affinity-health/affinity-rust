pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientRequestClinicalProfileWeightPounds {
    Double(f64),

    CreatePatientRequestClinicalProfileWeightPoundsOne(
        CreatePatientRequestClinicalProfileWeightPoundsOne,
    ),
}

impl CreatePatientRequestClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_request_clinical_profile_weight_pounds_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientRequestClinicalProfileWeightPoundsOne(_)
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

    pub fn as_create_patient_request_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&CreatePatientRequestClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreatePatientRequestClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_request_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<CreatePatientRequestClinicalProfileWeightPoundsOne> {
        match self {
            Self::CreatePatientRequestClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientRequestClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientRequestClinicalProfileWeightPoundsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

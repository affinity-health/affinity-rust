pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPatientResponseClinicalProfileWeightPounds {
    Double(f64),

    GetPatientResponseClinicalProfileWeightPoundsOne(
        GetPatientResponseClinicalProfileWeightPoundsOne,
    ),
}

impl GetPatientResponseClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_patient_response_clinical_profile_weight_pounds_one(&self) -> bool {
        matches!(
            self,
            Self::GetPatientResponseClinicalProfileWeightPoundsOne(_)
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

    pub fn as_get_patient_response_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&GetPatientResponseClinicalProfileWeightPoundsOne> {
        match self {
            Self::GetPatientResponseClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_patient_response_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<GetPatientResponseClinicalProfileWeightPoundsOne> {
        match self {
            Self::GetPatientResponseClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPatientResponseClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPatientResponseClinicalProfileWeightPoundsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

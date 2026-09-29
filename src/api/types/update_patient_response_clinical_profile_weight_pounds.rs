pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdatePatientResponseClinicalProfileWeightPounds {
    Double(f64),

    UpdatePatientResponseClinicalProfileWeightPoundsOne(
        UpdatePatientResponseClinicalProfileWeightPoundsOne,
    ),
}

impl UpdatePatientResponseClinicalProfileWeightPounds {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_patient_response_clinical_profile_weight_pounds_one(&self) -> bool {
        matches!(
            self,
            Self::UpdatePatientResponseClinicalProfileWeightPoundsOne(_)
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

    pub fn as_update_patient_response_clinical_profile_weight_pounds_one(
        &self,
    ) -> Option<&UpdatePatientResponseClinicalProfileWeightPoundsOne> {
        match self {
            Self::UpdatePatientResponseClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_patient_response_clinical_profile_weight_pounds_one(
        self,
    ) -> Option<UpdatePatientResponseClinicalProfileWeightPoundsOne> {
        match self {
            Self::UpdatePatientResponseClinicalProfileWeightPoundsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdatePatientResponseClinicalProfileWeightPounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdatePatientResponseClinicalProfileWeightPoundsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPracticeTeamResponsePrescribersActive {
    Double(f64),

    GetPracticeTeamResponsePrescribersActiveOne(GetPracticeTeamResponsePrescribersActiveOne),
}

impl GetPracticeTeamResponsePrescribersActive {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_practice_team_response_prescribers_active_one(&self) -> bool {
        matches!(self, Self::GetPracticeTeamResponsePrescribersActiveOne(_))
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

    pub fn as_get_practice_team_response_prescribers_active_one(
        &self,
    ) -> Option<&GetPracticeTeamResponsePrescribersActiveOne> {
        match self {
            Self::GetPracticeTeamResponsePrescribersActiveOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_practice_team_response_prescribers_active_one(
        self,
    ) -> Option<GetPracticeTeamResponsePrescribersActiveOne> {
        match self {
            Self::GetPracticeTeamResponsePrescribersActiveOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPracticeTeamResponsePrescribersActive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPracticeTeamResponsePrescribersActiveOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

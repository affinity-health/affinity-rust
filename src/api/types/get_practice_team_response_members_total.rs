pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPracticeTeamResponseMembersTotal {
    Double(f64),

    GetPracticeTeamResponseMembersTotalOne(GetPracticeTeamResponseMembersTotalOne),
}

impl GetPracticeTeamResponseMembersTotal {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_practice_team_response_members_total_one(&self) -> bool {
        matches!(self, Self::GetPracticeTeamResponseMembersTotalOne(_))
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

    pub fn as_get_practice_team_response_members_total_one(
        &self,
    ) -> Option<&GetPracticeTeamResponseMembersTotalOne> {
        match self {
            Self::GetPracticeTeamResponseMembersTotalOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_practice_team_response_members_total_one(
        self,
    ) -> Option<GetPracticeTeamResponseMembersTotalOne> {
        match self {
            Self::GetPracticeTeamResponseMembersTotalOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPracticeTeamResponseMembersTotal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPracticeTeamResponseMembersTotalOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

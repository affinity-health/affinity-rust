pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPracticeTeamResponseInvitationsPending {
    Double(f64),

    GetPracticeTeamResponseInvitationsPendingOne(GetPracticeTeamResponseInvitationsPendingOne),
}

impl GetPracticeTeamResponseInvitationsPending {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_practice_team_response_invitations_pending_one(&self) -> bool {
        matches!(self, Self::GetPracticeTeamResponseInvitationsPendingOne(_))
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

    pub fn as_get_practice_team_response_invitations_pending_one(
        &self,
    ) -> Option<&GetPracticeTeamResponseInvitationsPendingOne> {
        match self {
            Self::GetPracticeTeamResponseInvitationsPendingOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_practice_team_response_invitations_pending_one(
        self,
    ) -> Option<GetPracticeTeamResponseInvitationsPendingOne> {
        match self {
            Self::GetPracticeTeamResponseInvitationsPendingOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPracticeTeamResponseInvitationsPending {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPracticeTeamResponseInvitationsPendingOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

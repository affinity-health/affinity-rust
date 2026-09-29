pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetPracticeTeamInvitationResponsePersonObject {
    #[serde(rename = "team_person")]
    TeamPerson,
}
impl fmt::Display for GetPracticeTeamInvitationResponsePersonObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TeamPerson => "team_person",
        };
        write!(f, "{}", s)
    }
}

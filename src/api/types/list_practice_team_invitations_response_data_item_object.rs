pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPracticeTeamInvitationsResponseDataItemObject {
    #[serde(rename = "team_invitation")]
    TeamInvitation,
}
impl fmt::Display for ListPracticeTeamInvitationsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TeamInvitation => "team_invitation",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPracticeTeamInvitationsResponseDataItemPersonObject {
    #[serde(rename = "team_person")]
    TeamPerson,
}
impl fmt::Display for ListPracticeTeamInvitationsResponseDataItemPersonObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TeamPerson => "team_person",
        };
        write!(f, "{}", s)
    }
}

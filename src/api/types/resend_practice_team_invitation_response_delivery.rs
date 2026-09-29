pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResendPracticeTeamInvitationResponseDelivery {
    #[serde(rename = "sent")]
    Sent,
}
impl fmt::Display for ResendPracticeTeamInvitationResponseDelivery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Sent => "sent",
        };
        write!(f, "{}", s)
    }
}

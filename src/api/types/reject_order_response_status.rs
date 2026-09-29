pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RejectOrderResponseStatus {
    #[serde(rename = "rejected")]
    Rejected,
}
impl fmt::Display for RejectOrderResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Rejected => "rejected",
        };
        write!(f, "{}", s)
    }
}

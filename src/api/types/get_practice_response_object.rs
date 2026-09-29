pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetPracticeResponseObject {
    #[serde(rename = "practice")]
    Practice,
}
impl fmt::Display for GetPracticeResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Practice => "practice",
        };
        write!(f, "{}", s)
    }
}

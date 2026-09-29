pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdatePracticeResponseObject {
    #[serde(rename = "practice")]
    Practice,
}
impl fmt::Display for UpdatePracticeResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Practice => "practice",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPracticesResponseUrl {
    #[serde(rename = "/v1/practices")]
    V1Practices,
}
impl fmt::Display for ListPracticesResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1Practices => "/v1/practices",
        };
        write!(f, "{}", s)
    }
}

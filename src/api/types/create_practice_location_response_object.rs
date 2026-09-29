pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePracticeLocationResponseObject {
    #[serde(rename = "location")]
    Location,
}
impl fmt::Display for CreatePracticeLocationResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Location => "location",
        };
        write!(f, "{}", s)
    }
}

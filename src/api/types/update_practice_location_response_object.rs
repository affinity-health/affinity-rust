pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdatePracticeLocationResponseObject {
    #[serde(rename = "location")]
    Location,
}
impl fmt::Display for UpdatePracticeLocationResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Location => "location",
        };
        write!(f, "{}", s)
    }
}

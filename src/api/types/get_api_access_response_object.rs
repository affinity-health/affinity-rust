pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetApiAccessResponseObject {
    #[serde(rename = "api_access")]
    ApiAccess,
}
impl fmt::Display for GetApiAccessResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ApiAccess => "api_access",
        };
        write!(f, "{}", s)
    }
}

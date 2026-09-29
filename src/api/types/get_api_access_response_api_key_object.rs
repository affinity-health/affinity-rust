pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetApiAccessResponseApiKeyObject {
    #[serde(rename = "api_key")]
    ApiKey,
}
impl fmt::Display for GetApiAccessResponseApiKeyObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ApiKey => "api_key",
        };
        write!(f, "{}", s)
    }
}

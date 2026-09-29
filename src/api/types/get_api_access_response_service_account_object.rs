pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetApiAccessResponseServiceAccountObject {
    #[serde(rename = "service_account")]
    ServiceAccount,
}
impl fmt::Display for GetApiAccessResponseServiceAccountObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ServiceAccount => "service_account",
        };
        write!(f, "{}", s)
    }
}

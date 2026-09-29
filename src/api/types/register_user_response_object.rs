pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RegisterUserResponseObject {
    #[serde(rename = "registered_user")]
    RegisteredUser,
}
impl fmt::Display for RegisterUserResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RegisteredUser => "registered_user",
        };
        write!(f, "{}", s)
    }
}

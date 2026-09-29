pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubmitOrderResponseStatus {
    #[serde(rename = "submitted")]
    Submitted,
}
impl fmt::Display for SubmitOrderResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Submitted => "submitted",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SignOrderResponseStatus {
    #[serde(rename = "ready")]
    Ready,
}
impl fmt::Display for SignOrderResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ready => "ready",
        };
        write!(f, "{}", s)
    }
}

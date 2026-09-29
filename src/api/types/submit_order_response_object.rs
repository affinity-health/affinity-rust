pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubmitOrderResponseObject {
    #[serde(rename = "order_submission")]
    OrderSubmission,
}
impl fmt::Display for SubmitOrderResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OrderSubmission => "order_submission",
        };
        write!(f, "{}", s)
    }
}

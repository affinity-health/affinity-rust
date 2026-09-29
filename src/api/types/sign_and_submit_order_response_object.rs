pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SignAndSubmitOrderResponseObject {
    #[serde(rename = "order_sign_and_submission")]
    OrderSignAndSubmission,
}
impl fmt::Display for SignAndSubmitOrderResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OrderSignAndSubmission => "order_sign_and_submission",
        };
        write!(f, "{}", s)
    }
}

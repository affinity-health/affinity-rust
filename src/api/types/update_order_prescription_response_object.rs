pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateOrderPrescriptionResponseObject {
    #[serde(rename = "order_draft_update")]
    OrderDraftUpdate,
}
impl fmt::Display for UpdateOrderPrescriptionResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OrderDraftUpdate => "order_draft_update",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderBatchResponseObject {
    #[serde(rename = "order_batch")]
    OrderBatch,
}
impl fmt::Display for CreateOrderBatchResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OrderBatch => "order_batch",
        };
        write!(f, "{}", s)
    }
}

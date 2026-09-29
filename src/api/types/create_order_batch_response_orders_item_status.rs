pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderBatchResponseOrdersItemStatus {
    #[serde(rename = "requires_provider_signature")]
    RequiresProviderSignature,
}
impl fmt::Display for CreateOrderBatchResponseOrdersItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RequiresProviderSignature => "requires_provider_signature",
        };
        write!(f, "{}", s)
    }
}

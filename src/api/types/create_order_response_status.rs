pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderResponseStatus {
    #[serde(rename = "requires_provider_signature")]
    RequiresProviderSignature,
}
impl fmt::Display for CreateOrderResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::RequiresProviderSignature => "requires_provider_signature",
        };
        write!(f, "{}", s)
    }
}

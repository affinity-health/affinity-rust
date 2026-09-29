pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisItemQuantity {
    #[serde(rename = "1")]
    One,
}
impl fmt::Display for PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::One => "1",
        };
        write!(f, "{}", s)
    }
}

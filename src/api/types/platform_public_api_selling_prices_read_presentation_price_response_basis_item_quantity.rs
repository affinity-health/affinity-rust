pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantity {
    #[serde(rename = "1")]
    One,
}
impl fmt::Display for PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::One => "1",
        };
        write!(f, "{}", s)
    }
}

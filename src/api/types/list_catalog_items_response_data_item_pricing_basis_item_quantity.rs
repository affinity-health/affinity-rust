pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListCatalogItemsResponseDataItemPricingBasisItemQuantity {
    #[serde(rename = "1")]
    One,
}
impl fmt::Display for ListCatalogItemsResponseDataItemPricingBasisItemQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::One => "1",
        };
        write!(f, "{}", s)
    }
}

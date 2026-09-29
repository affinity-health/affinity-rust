pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListCatalogItemsResponseDataItemFulfillmentInclusionsItemPriceComponent {
    #[serde(rename = "shipping")]
    Shipping,
}
impl fmt::Display for ListCatalogItemsResponseDataItemFulfillmentInclusionsItemPriceComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Shipping => "shipping",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemPriceComponent {
    #[serde(rename = "shipping")]
    Shipping,
}
impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemPriceComponent
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Shipping => "shipping",
        };
        write!(f, "{}", s)
    }
}

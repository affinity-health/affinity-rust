pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogPricingCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl fmt::Display for RetrievePrescribingOptionsResponseCatalogPricingCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Usd => "USD",
        };
        write!(f, "{}", s)
    }
}

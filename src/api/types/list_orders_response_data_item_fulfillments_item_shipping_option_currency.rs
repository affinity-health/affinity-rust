pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseDataItemFulfillmentsItemShippingOptionCurrency {
    #[serde(rename = "USD")]
    Usd,
}
impl fmt::Display for ListOrdersResponseDataItemFulfillmentsItemShippingOptionCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Usd => "USD",
        };
        write!(f, "{}", s)
    }
}

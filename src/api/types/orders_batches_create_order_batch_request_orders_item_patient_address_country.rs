pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderBatchRequestOrdersItemPatientAddressCountry {
    #[serde(rename = "US")]
    Us,
}
impl fmt::Display for CreateOrderBatchRequestOrdersItemPatientAddressCountry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Us => "US",
        };
        write!(f, "{}", s)
    }
}

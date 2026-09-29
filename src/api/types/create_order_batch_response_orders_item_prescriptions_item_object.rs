pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderBatchResponseOrdersItemPrescriptionsItemObject {
    #[serde(rename = "prescription")]
    Prescription,
}
impl fmt::Display for CreateOrderBatchResponseOrdersItemPrescriptionsItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Prescription => "prescription",
        };
        write!(f, "{}", s)
    }
}

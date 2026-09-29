pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseMedicationRxnormSystem {
    #[serde(rename = "rxnorm")]
    Rxnorm,
}
impl fmt::Display for RetrievePrescribingOptionsResponseMedicationRxnormSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Rxnorm => "rxnorm",
        };
        write!(f, "{}", s)
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetPatientAllergiesResponseAllergiesItemReactionsItemCodeSystem {
    #[serde(rename = "snomed-ct")]
    SnomedCt,
}
impl fmt::Display for GetPatientAllergiesResponseAllergiesItemReactionsItemCodeSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SnomedCt => "snomed-ct",
        };
        write!(f, "{}", s)
    }
}

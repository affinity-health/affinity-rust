pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReplacePatientAllergiesResponseAllergiesItemReactionsItemCodeSystem {
    #[serde(rename = "snomed-ct")]
    SnomedCt,
}
impl fmt::Display for ReplacePatientAllergiesResponseAllergiesItemReactionsItemCodeSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SnomedCt => "snomed-ct",
        };
        write!(f, "{}", s)
    }
}

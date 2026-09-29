pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseObject {
    #[serde(rename = "prescribing_options")]
    PrescribingOptions,
}
impl fmt::Display for RetrievePrescribingOptionsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PrescribingOptions => "prescribing_options",
        };
        write!(f, "{}", s)
    }
}

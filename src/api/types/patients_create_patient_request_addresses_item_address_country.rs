pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePatientRequestAddressesItemAddressCountry {
    #[serde(rename = "US")]
    Us,
}
impl fmt::Display for CreatePatientRequestAddressesItemAddressCountry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Us => "US",
        };
        write!(f, "{}", s)
    }
}

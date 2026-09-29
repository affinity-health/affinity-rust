pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPharmaciesResponseUrl {
    #[serde(rename = "/v1/pharmacies")]
    V1Pharmacies,
}
impl fmt::Display for ListPharmaciesResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1Pharmacies => "/v1/pharmacies",
        };
        write!(f, "{}", s)
    }
}

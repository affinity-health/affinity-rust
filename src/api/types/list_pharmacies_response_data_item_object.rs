pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListPharmaciesResponseDataItemObject {
    #[serde(rename = "pharmacy")]
    Pharmacy,
}
impl fmt::Display for ListPharmaciesResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pharmacy => "pharmacy",
        };
        write!(f, "{}", s)
    }
}

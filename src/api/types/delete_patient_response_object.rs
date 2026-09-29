pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DeletePatientResponseObject {
    #[serde(rename = "patient")]
    Patient,
}
impl fmt::Display for DeletePatientResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Patient => "patient",
        };
        write!(f, "{}", s)
    }
}

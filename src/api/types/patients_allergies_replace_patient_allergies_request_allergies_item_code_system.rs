pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplacePatientAllergiesRequestAllergiesItemCodeSystem {
    Rxnorm,
    SnomedCt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplacePatientAllergiesRequestAllergiesItemCodeSystem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rxnorm => serializer.serialize_str("rxnorm"),
            Self::SnomedCt => serializer.serialize_str("snomed-ct"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplacePatientAllergiesRequestAllergiesItemCodeSystem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rxnorm" => Ok(Self::Rxnorm),
            "snomed-ct" => Ok(Self::SnomedCt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplacePatientAllergiesRequestAllergiesItemCodeSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rxnorm => write!(f, "rxnorm"),
            Self::SnomedCt => write!(f, "snomed-ct"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

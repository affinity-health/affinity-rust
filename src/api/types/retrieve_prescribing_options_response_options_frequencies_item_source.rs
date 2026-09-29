pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource {
    Catalog,
    Pharmacy,
    Rxnorm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Catalog => serializer.serialize_str("catalog"),
            Self::Pharmacy => serializer.serialize_str("pharmacy"),
            Self::Rxnorm => serializer.serialize_str("rxnorm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "catalog" => Ok(Self::Catalog),
            "pharmacy" => Ok(Self::Pharmacy),
            "rxnorm" => Ok(Self::Rxnorm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Catalog => write!(f, "catalog"),
            Self::Pharmacy => write!(f, "pharmacy"),
            Self::Rxnorm => write!(f, "rxnorm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

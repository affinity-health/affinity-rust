pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponsePresetsItemSource {
    Affinity,
    Pharmacy,
    Catalog,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponsePresetsItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Affinity => serializer.serialize_str("affinity"),
            Self::Pharmacy => serializer.serialize_str("pharmacy"),
            Self::Catalog => serializer.serialize_str("catalog"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePrescribingOptionsResponsePresetsItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "affinity" => Ok(Self::Affinity),
            "pharmacy" => Ok(Self::Pharmacy),
            "catalog" => Ok(Self::Catalog),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponsePresetsItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Affinity => write!(f, "affinity"),
            Self::Pharmacy => write!(f, "pharmacy"),
            Self::Catalog => write!(f, "catalog"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

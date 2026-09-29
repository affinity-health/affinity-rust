pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogCompositionStatus {
    Complete,
    Partial,
    Unresolved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponseCatalogCompositionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Complete => serializer.serialize_str("complete"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePrescribingOptionsResponseCatalogCompositionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "complete" => Ok(Self::Complete),
            "partial" => Ok(Self::Partial),
            "unresolved" => Ok(Self::Unresolved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseCatalogCompositionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => write!(f, "complete"),
            Self::Partial => write!(f, "partial"),
            Self::Unresolved => write!(f, "unresolved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

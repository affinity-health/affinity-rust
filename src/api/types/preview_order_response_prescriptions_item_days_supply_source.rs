pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderResponsePrescriptionsItemDaysSupplySource {
    Manual,
    Calculated,
    Preset,
    Missing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderResponsePrescriptionsItemDaysSupplySource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::Calculated => serializer.serialize_str("calculated"),
            Self::Preset => serializer.serialize_str("preset"),
            Self::Missing => serializer.serialize_str("missing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreviewOrderResponsePrescriptionsItemDaysSupplySource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "calculated" => Ok(Self::Calculated),
            "preset" => Ok(Self::Preset),
            "missing" => Ok(Self::Missing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreviewOrderResponsePrescriptionsItemDaysSupplySource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::Calculated => write!(f, "calculated"),
            Self::Preset => write!(f, "preset"),
            Self::Missing => write!(f, "missing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

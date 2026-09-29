pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderRequestShippingSelection {
    Manual,
    LowestCost,
    Fastest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderRequestShippingSelection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::LowestCost => serializer.serialize_str("lowest_cost"),
            Self::Fastest => serializer.serialize_str("fastest"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreviewOrderRequestShippingSelection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "lowest_cost" => Ok(Self::LowestCost),
            "fastest" => Ok(Self::Fastest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreviewOrderRequestShippingSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::LowestCost => write!(f, "lowest_cost"),
            Self::Fastest => write!(f, "fastest"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

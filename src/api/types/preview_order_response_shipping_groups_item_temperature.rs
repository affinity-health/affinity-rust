pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderResponseShippingGroupsItemTemperature {
    Ambient,
    Refrigerated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderResponseShippingGroupsItemTemperature {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ambient => serializer.serialize_str("ambient"),
            Self::Refrigerated => serializer.serialize_str("refrigerated"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreviewOrderResponseShippingGroupsItemTemperature {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ambient" => Ok(Self::Ambient),
            "refrigerated" => Ok(Self::Refrigerated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreviewOrderResponseShippingGroupsItemTemperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ambient => write!(f, "ambient"),
            Self::Refrigerated => write!(f, "refrigerated"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

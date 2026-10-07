pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne {
    NaN,
    Infinity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NaN => serializer.serialize_str("NaN"),
            Self::Infinity => serializer.serialize_str("Infinity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NaN" => Ok(Self::NaN),
            "Infinity" => Ok(Self::Infinity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SignAndSubmitOrderResponsePrescriptionsItemErrorStatusOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NaN => write!(f, "NaN"),
            Self::Infinity => write!(f, "Infinity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

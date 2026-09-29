pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetOrderResponsePrescriptionsItemDispensingShippingDestinationType {
    Patient,
    Practice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetOrderResponsePrescriptionsItemDispensingShippingDestinationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Patient => serializer.serialize_str("patient"),
            Self::Practice => serializer.serialize_str("practice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetOrderResponsePrescriptionsItemDispensingShippingDestinationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "patient" => Ok(Self::Patient),
            "practice" => Ok(Self::Practice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetOrderResponsePrescriptionsItemDispensingShippingDestinationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Patient => write!(f, "patient"),
            Self::Practice => write!(f, "practice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

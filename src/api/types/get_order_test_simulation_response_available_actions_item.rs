pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetOrderTestSimulationResponseAvailableActionsItem {
    Ship,
    Deliver,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetOrderTestSimulationResponseAvailableActionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ship => serializer.serialize_str("ship"),
            Self::Deliver => serializer.serialize_str("deliver"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetOrderTestSimulationResponseAvailableActionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ship" => Ok(Self::Ship),
            "deliver" => Ok(Self::Deliver),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetOrderTestSimulationResponseAvailableActionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ship => write!(f, "ship"),
            Self::Deliver => write!(f, "deliver"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

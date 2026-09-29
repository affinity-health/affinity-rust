pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetOrderTestSimulationResponseScenario {
    Successful,
    PharmacyRejection,
    CancellationDeclined,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetOrderTestSimulationResponseScenario {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Successful => serializer.serialize_str("successful"),
            Self::PharmacyRejection => serializer.serialize_str("pharmacy_rejection"),
            Self::CancellationDeclined => serializer.serialize_str("cancellation_declined"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetOrderTestSimulationResponseScenario {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "successful" => Ok(Self::Successful),
            "pharmacy_rejection" => Ok(Self::PharmacyRejection),
            "cancellation_declined" => Ok(Self::CancellationDeclined),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetOrderTestSimulationResponseScenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Successful => write!(f, "successful"),
            Self::PharmacyRejection => write!(f, "pharmacy_rejection"),
            Self::CancellationDeclined => write!(f, "cancellation_declined"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

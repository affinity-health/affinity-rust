pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelOrderResponseFulfillmentsItemCancellationsItemSource {
    Provider,
    Platform,
    PublicApi,
    Pharmacy,
    System,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelOrderResponseFulfillmentsItemCancellationsItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Provider => serializer.serialize_str("provider"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::PublicApi => serializer.serialize_str("public_api"),
            Self::Pharmacy => serializer.serialize_str("pharmacy"),
            Self::System => serializer.serialize_str("system"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelOrderResponseFulfillmentsItemCancellationsItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "provider" => Ok(Self::Provider),
            "platform" => Ok(Self::Platform),
            "public_api" => Ok(Self::PublicApi),
            "pharmacy" => Ok(Self::Pharmacy),
            "system" => Ok(Self::System),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelOrderResponseFulfillmentsItemCancellationsItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Provider => write!(f, "provider"),
            Self::Platform => write!(f, "platform"),
            Self::PublicApi => write!(f, "public_api"),
            Self::Pharmacy => write!(f, "pharmacy"),
            Self::System => write!(f, "system"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

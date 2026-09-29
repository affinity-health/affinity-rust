pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEventsRequestStatus {
    All,
    Delivered,
    Failed,
    Pending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEventsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEventsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "pending" => Ok(Self::Pending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEventsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Delivered => write!(f, "delivered"),
            Self::Failed => write!(f, "failed"),
            Self::Pending => write!(f, "pending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

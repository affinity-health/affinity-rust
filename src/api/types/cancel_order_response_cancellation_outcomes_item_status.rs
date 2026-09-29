pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelOrderResponseCancellationOutcomesItemStatus {
    Confirmed,
    Failed,
    Rejected,
    Requested,
    Sent,
    TooLate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelOrderResponseCancellationOutcomesItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Requested => serializer.serialize_str("requested"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::TooLate => serializer.serialize_str("too_late"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelOrderResponseCancellationOutcomesItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "confirmed" => Ok(Self::Confirmed),
            "failed" => Ok(Self::Failed),
            "rejected" => Ok(Self::Rejected),
            "requested" => Ok(Self::Requested),
            "sent" => Ok(Self::Sent),
            "too_late" => Ok(Self::TooLate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelOrderResponseCancellationOutcomesItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed => write!(f, "confirmed"),
            Self::Failed => write!(f, "failed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Requested => write!(f, "requested"),
            Self::Sent => write!(f, "sent"),
            Self::TooLate => write!(f, "too_late"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

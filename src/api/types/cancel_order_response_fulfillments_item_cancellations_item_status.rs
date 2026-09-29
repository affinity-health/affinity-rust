pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelOrderResponseFulfillmentsItemCancellationsItemStatus {
    Requested,
    Sent,
    Confirmed,
    Rejected,
    Failed,
    TooLate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelOrderResponseFulfillmentsItemCancellationsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::TooLate => serializer.serialize_str("too_late"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelOrderResponseFulfillmentsItemCancellationsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "sent" => Ok(Self::Sent),
            "confirmed" => Ok(Self::Confirmed),
            "rejected" => Ok(Self::Rejected),
            "failed" => Ok(Self::Failed),
            "too_late" => Ok(Self::TooLate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelOrderResponseFulfillmentsItemCancellationsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Sent => write!(f, "sent"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Failed => write!(f, "failed"),
            Self::TooLate => write!(f, "too_late"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

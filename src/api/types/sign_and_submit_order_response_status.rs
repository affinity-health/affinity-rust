pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignAndSubmitOrderResponseStatus {
    Submitted,
    PartiallySubmitted,
    NotSubmitted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SignAndSubmitOrderResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::PartiallySubmitted => serializer.serialize_str("partially_submitted"),
            Self::NotSubmitted => serializer.serialize_str("not_submitted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SignAndSubmitOrderResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "submitted" => Ok(Self::Submitted),
            "partially_submitted" => Ok(Self::PartiallySubmitted),
            "not_submitted" => Ok(Self::NotSubmitted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SignAndSubmitOrderResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "submitted"),
            Self::PartiallySubmitted => write!(f, "partially_submitted"),
            Self::NotSubmitted => write!(f, "not_submitted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

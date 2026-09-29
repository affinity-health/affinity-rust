pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseDataItemStatus {
    Blocked,
    Cancelled,
    Delivered,
    Draft,
    PartiallySubmitted,
    RequiresProviderSignature,
    Processing,
    Ready,
    Rejected,
    Shipped,
    Submitted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOrdersResponseDataItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blocked => serializer.serialize_str("blocked"),
            Self::Cancelled => serializer.serialize_str("cancelled"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::PartiallySubmitted => serializer.serialize_str("partially_submitted"),
            Self::RequiresProviderSignature => {
                serializer.serialize_str("requires_provider_signature")
            }
            Self::Processing => serializer.serialize_str("processing"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Shipped => serializer.serialize_str("shipped"),
            Self::Submitted => serializer.serialize_str("submitted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListOrdersResponseDataItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blocked" => Ok(Self::Blocked),
            "cancelled" => Ok(Self::Cancelled),
            "delivered" => Ok(Self::Delivered),
            "draft" => Ok(Self::Draft),
            "partially_submitted" => Ok(Self::PartiallySubmitted),
            "requires_provider_signature" => Ok(Self::RequiresProviderSignature),
            "processing" => Ok(Self::Processing),
            "ready" => Ok(Self::Ready),
            "rejected" => Ok(Self::Rejected),
            "shipped" => Ok(Self::Shipped),
            "submitted" => Ok(Self::Submitted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListOrdersResponseDataItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blocked => write!(f, "blocked"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Delivered => write!(f, "delivered"),
            Self::Draft => write!(f, "draft"),
            Self::PartiallySubmitted => write!(f, "partially_submitted"),
            Self::RequiresProviderSignature => write!(f, "requires_provider_signature"),
            Self::Processing => write!(f, "processing"),
            Self::Ready => write!(f, "ready"),
            Self::Rejected => write!(f, "rejected"),
            Self::Shipped => write!(f, "shipped"),
            Self::Submitted => write!(f, "submitted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

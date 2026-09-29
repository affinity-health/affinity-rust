pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemsRequestAvailability {
    All,
    Orderable,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListItemsRequestAvailability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Orderable => serializer.serialize_str("orderable"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListItemsRequestAvailability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "orderable" => Ok(Self::Orderable),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListItemsRequestAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Orderable => write!(f, "orderable"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

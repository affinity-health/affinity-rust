pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAddressesRequestStatus {
    Active,
    Archived,
    All,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAddressesRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::All => serializer.serialize_str("all"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAddressesRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            "all" => Ok(Self::All),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAddressesRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Archived => write!(f, "archived"),
            Self::All => write!(f, "all"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

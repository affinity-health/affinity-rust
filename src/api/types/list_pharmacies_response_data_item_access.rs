pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPharmaciesResponseDataItemAccess {
    Invited,
    Network,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPharmaciesResponseDataItemAccess {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Invited => serializer.serialize_str("invited"),
            Self::Network => serializer.serialize_str("network"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPharmaciesResponseDataItemAccess {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invited" => Ok(Self::Invited),
            "network" => Ok(Self::Network),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPharmaciesResponseDataItemAccess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invited => write!(f, "invited"),
            Self::Network => write!(f, "network"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

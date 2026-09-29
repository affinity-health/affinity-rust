pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemsRequestRoutesOneItem {
    Injectable,
    Nasal,
    Oral,
    Sublingual,
    Topical,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListItemsRequestRoutesOneItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Injectable => serializer.serialize_str("injectable"),
            Self::Nasal => serializer.serialize_str("nasal"),
            Self::Oral => serializer.serialize_str("oral"),
            Self::Sublingual => serializer.serialize_str("sublingual"),
            Self::Topical => serializer.serialize_str("topical"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListItemsRequestRoutesOneItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "injectable" => Ok(Self::Injectable),
            "nasal" => Ok(Self::Nasal),
            "oral" => Ok(Self::Oral),
            "sublingual" => Ok(Self::Sublingual),
            "topical" => Ok(Self::Topical),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListItemsRequestRoutesOneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Injectable => write!(f, "injectable"),
            Self::Nasal => write!(f, "nasal"),
            Self::Oral => write!(f, "oral"),
            Self::Sublingual => write!(f, "sublingual"),
            Self::Topical => write!(f, "topical"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

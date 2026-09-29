pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemsRequestSort {
    Relevance,
    NameAsc,
    NameDesc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListItemsRequestSort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Relevance => serializer.serialize_str("relevance"),
            Self::NameAsc => serializer.serialize_str("name_asc"),
            Self::NameDesc => serializer.serialize_str("name_desc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListItemsRequestSort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "relevance" => Ok(Self::Relevance),
            "name_asc" => Ok(Self::NameAsc),
            "name_desc" => Ok(Self::NameDesc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListItemsRequestSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relevance => write!(f, "relevance"),
            Self::NameAsc => write!(f, "name_asc"),
            Self::NameDesc => write!(f, "name_desc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOrdersRequestSort {
    Newest,
    Oldest,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOrdersRequestSort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Newest => serializer.serialize_str("newest"),
            Self::Oldest => serializer.serialize_str("oldest"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListOrdersRequestSort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "newest" => Ok(Self::Newest),
            "oldest" => Ok(Self::Oldest),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListOrdersRequestSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Newest => write!(f, "newest"),
            Self::Oldest => write!(f, "oldest"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

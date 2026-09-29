pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemsRequestCatalogKind {
    Prescription,
    Otc,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListItemsRequestCatalogKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Prescription => serializer.serialize_str("prescription"),
            Self::Otc => serializer.serialize_str("otc"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListItemsRequestCatalogKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prescription" => Ok(Self::Prescription),
            "otc" => Ok(Self::Otc),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListItemsRequestCatalogKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prescription => write!(f, "prescription"),
            Self::Otc => write!(f, "otc"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

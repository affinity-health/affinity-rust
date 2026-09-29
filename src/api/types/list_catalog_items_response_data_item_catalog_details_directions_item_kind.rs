pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind {
    Suggested,
    Template,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Suggested => serializer.serialize_str("suggested"),
            Self::Template => serializer.serialize_str("template"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "suggested" => Ok(Self::Suggested),
            "template" => Ok(Self::Template),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Suggested => write!(f, "suggested"),
            Self::Template => write!(f, "template"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

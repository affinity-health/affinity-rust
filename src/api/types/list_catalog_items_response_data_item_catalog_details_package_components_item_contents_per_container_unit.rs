pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit
{
    Mg,
    G,
    Ug,
    ML,
    L,
    Iu,
    Tablet,
    Capsule,
    Troche,
    Actuation,
    Patch,
    Package,
    Container,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize
    for ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mg => serializer.serialize_str("mg"),
            Self::G => serializer.serialize_str("g"),
            Self::Ug => serializer.serialize_str("ug"),
            Self::ML => serializer.serialize_str("mL"),
            Self::L => serializer.serialize_str("L"),
            Self::Iu => serializer.serialize_str("[IU]"),
            Self::Tablet => serializer.serialize_str("tablet"),
            Self::Capsule => serializer.serialize_str("capsule"),
            Self::Troche => serializer.serialize_str("troche"),
            Self::Actuation => serializer.serialize_str("actuation"),
            Self::Patch => serializer.serialize_str("patch"),
            Self::Package => serializer.serialize_str("package"),
            Self::Container => serializer.serialize_str("container"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mg" => Ok(Self::Mg),
            "g" => Ok(Self::G),
            "ug" => Ok(Self::Ug),
            "mL" => Ok(Self::ML),
            "L" => Ok(Self::L),
            "[IU]" => Ok(Self::Iu),
            "tablet" => Ok(Self::Tablet),
            "capsule" => Ok(Self::Capsule),
            "troche" => Ok(Self::Troche),
            "actuation" => Ok(Self::Actuation),
            "patch" => Ok(Self::Patch),
            "package" => Ok(Self::Package),
            "container" => Ok(Self::Container),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mg => write!(f, "mg"),
            Self::G => write!(f, "g"),
            Self::Ug => write!(f, "ug"),
            Self::ML => write!(f, "mL"),
            Self::L => write!(f, "L"),
            Self::Iu => write!(f, "[IU]"),
            Self::Tablet => write!(f, "tablet"),
            Self::Capsule => write!(f, "capsule"),
            Self::Troche => write!(f, "troche"),
            Self::Actuation => write!(f, "actuation"),
            Self::Patch => write!(f, "patch"),
            Self::Package => write!(f, "package"),
            Self::Container => write!(f, "container"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

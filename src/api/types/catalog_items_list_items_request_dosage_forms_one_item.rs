pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListItemsRequestDosageFormsOneItem {
    Capsule,
    Cream,
    Gel,
    Solution,
    Spray,
    Tablet,
    Troche,
    Unknown,
    Vial,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListItemsRequestDosageFormsOneItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Capsule => serializer.serialize_str("capsule"),
            Self::Cream => serializer.serialize_str("cream"),
            Self::Gel => serializer.serialize_str("gel"),
            Self::Solution => serializer.serialize_str("solution"),
            Self::Spray => serializer.serialize_str("spray"),
            Self::Tablet => serializer.serialize_str("tablet"),
            Self::Troche => serializer.serialize_str("troche"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::Vial => serializer.serialize_str("vial"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListItemsRequestDosageFormsOneItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "capsule" => Ok(Self::Capsule),
            "cream" => Ok(Self::Cream),
            "gel" => Ok(Self::Gel),
            "solution" => Ok(Self::Solution),
            "spray" => Ok(Self::Spray),
            "tablet" => Ok(Self::Tablet),
            "troche" => Ok(Self::Troche),
            "unknown" => Ok(Self::Unknown),
            "vial" => Ok(Self::Vial),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListItemsRequestDosageFormsOneItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capsule => write!(f, "capsule"),
            Self::Cream => write!(f, "cream"),
            Self::Gel => write!(f, "gel"),
            Self::Solution => write!(f, "solution"),
            Self::Spray => write!(f, "spray"),
            Self::Tablet => write!(f, "tablet"),
            Self::Troche => write!(f, "troche"),
            Self::Unknown => write!(f, "unknown"),
            Self::Vial => write!(f, "vial"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

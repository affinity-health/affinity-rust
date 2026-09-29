pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplacePatientAllergiesResponseAllergiesItemCategory {
    Drug,
    Food,
    Insect,
    Latex,
    Mold,
    Pet,
    Pollen,
    Environmental,
    Biologic,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplacePatientAllergiesResponseAllergiesItemCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Drug => serializer.serialize_str("drug"),
            Self::Food => serializer.serialize_str("food"),
            Self::Insect => serializer.serialize_str("insect"),
            Self::Latex => serializer.serialize_str("latex"),
            Self::Mold => serializer.serialize_str("mold"),
            Self::Pet => serializer.serialize_str("pet"),
            Self::Pollen => serializer.serialize_str("pollen"),
            Self::Environmental => serializer.serialize_str("environmental"),
            Self::Biologic => serializer.serialize_str("biologic"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplacePatientAllergiesResponseAllergiesItemCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "drug" => Ok(Self::Drug),
            "food" => Ok(Self::Food),
            "insect" => Ok(Self::Insect),
            "latex" => Ok(Self::Latex),
            "mold" => Ok(Self::Mold),
            "pet" => Ok(Self::Pet),
            "pollen" => Ok(Self::Pollen),
            "environmental" => Ok(Self::Environmental),
            "biologic" => Ok(Self::Biologic),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplacePatientAllergiesResponseAllergiesItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Drug => write!(f, "drug"),
            Self::Food => write!(f, "food"),
            Self::Insect => write!(f, "insect"),
            Self::Latex => write!(f, "latex"),
            Self::Mold => write!(f, "mold"),
            Self::Pet => write!(f, "pet"),
            Self::Pollen => write!(f, "pollen"),
            Self::Environmental => write!(f, "environmental"),
            Self::Biologic => write!(f, "biologic"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

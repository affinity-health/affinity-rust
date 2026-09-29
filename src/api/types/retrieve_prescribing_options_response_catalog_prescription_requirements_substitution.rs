pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution {
    NotSupported,
    Optional,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotSupported => serializer.serialize_str("not_supported"),
            Self::Optional => serializer.serialize_str("optional"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_supported" => Ok(Self::NotSupported),
            "optional" => Ok(Self::Optional),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "not_supported"),
            Self::Optional => write!(f, "optional"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

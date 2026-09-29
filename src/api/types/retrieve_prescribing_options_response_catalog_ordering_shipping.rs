pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogOrderingShipping {
    Prescription,
    AccompanyingPrescription,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponseCatalogOrderingShipping {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Prescription => serializer.serialize_str("prescription"),
            Self::AccompanyingPrescription => serializer.serialize_str("accompanying_prescription"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePrescribingOptionsResponseCatalogOrderingShipping {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prescription" => Ok(Self::Prescription),
            "accompanying_prescription" => Ok(Self::AccompanyingPrescription),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseCatalogOrderingShipping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prescription => write!(f, "prescription"),
            Self::AccompanyingPrescription => write!(f, "accompanying_prescription"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

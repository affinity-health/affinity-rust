pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule {
    Ii,
    Iii,
    Iv,
    V,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule
{
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ii => serializer.serialize_str("II"),
            Self::Iii => serializer.serialize_str("III"),
            Self::Iv => serializer.serialize_str("IV"),
            Self::V => serializer.serialize_str("V"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "II" => Ok(Self::Ii),
            "III" => Ok(Self::Iii),
            "IV" => Ok(Self::Iv),
            "V" => Ok(Self::V),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ii => write!(f, "II"),
            Self::Iii => write!(f, "III"),
            Self::Iv => write!(f, "IV"),
            Self::V => write!(f, "V"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

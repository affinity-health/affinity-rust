pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind {
    ColdChain,
    InjectionSupplies,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ColdChain => serializer.serialize_str("cold_chain"),
            Self::InjectionSupplies => serializer.serialize_str("injection_supplies"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cold_chain" => Ok(Self::ColdChain),
            "injection_supplies" => Ok(Self::InjectionSupplies),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ColdChain => write!(f, "cold_chain"),
            Self::InjectionSupplies => write!(f, "injection_supplies"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

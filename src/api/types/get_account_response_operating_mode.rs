pub use crate::prelude::*;

/// The organization's Live-access status, independent of this request's livemode.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAccountResponseOperatingMode {
    Production,
    ProductionPending,
    Sandbox,
    Suspended,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAccountResponseOperatingMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Production => serializer.serialize_str("production"),
            Self::ProductionPending => serializer.serialize_str("production_pending"),
            Self::Sandbox => serializer.serialize_str("sandbox"),
            Self::Suspended => serializer.serialize_str("suspended"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAccountResponseOperatingMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "production" => Ok(Self::Production),
            "production_pending" => Ok(Self::ProductionPending),
            "sandbox" => Ok(Self::Sandbox),
            "suspended" => Ok(Self::Suspended),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAccountResponseOperatingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Production => write!(f, "production"),
            Self::ProductionPending => write!(f, "production_pending"),
            Self::Sandbox => write!(f, "sandbox"),
            Self::Suspended => write!(f, "suspended"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

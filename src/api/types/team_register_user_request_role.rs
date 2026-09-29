pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegisterUserRequestRole {
    Administrator,
    Prescriber,
    ClinicalStaff,
    Billing,
    Developer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RegisterUserRequestRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Administrator => serializer.serialize_str("administrator"),
            Self::Prescriber => serializer.serialize_str("prescriber"),
            Self::ClinicalStaff => serializer.serialize_str("clinical_staff"),
            Self::Billing => serializer.serialize_str("billing"),
            Self::Developer => serializer.serialize_str("developer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RegisterUserRequestRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "administrator" => Ok(Self::Administrator),
            "prescriber" => Ok(Self::Prescriber),
            "clinical_staff" => Ok(Self::ClinicalStaff),
            "billing" => Ok(Self::Billing),
            "developer" => Ok(Self::Developer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RegisterUserRequestRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Administrator => write!(f, "administrator"),
            Self::Prescriber => write!(f, "prescriber"),
            Self::ClinicalStaff => write!(f, "clinical_staff"),
            Self::Billing => write!(f, "billing"),
            Self::Developer => write!(f, "developer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

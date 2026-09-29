pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetPatientAllergiesResponseAllergiesItemSource {
    Doctor,
    Patient,
    PatientAgentGuardian,
    Pharmacist,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetPatientAllergiesResponseAllergiesItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Doctor => serializer.serialize_str("Doctor"),
            Self::Patient => serializer.serialize_str("Patient"),
            Self::PatientAgentGuardian => serializer.serialize_str("Patient Agent/Guardian"),
            Self::Pharmacist => serializer.serialize_str("Pharmacist"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetPatientAllergiesResponseAllergiesItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Doctor" => Ok(Self::Doctor),
            "Patient" => Ok(Self::Patient),
            "Patient Agent/Guardian" => Ok(Self::PatientAgentGuardian),
            "Pharmacist" => Ok(Self::Pharmacist),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetPatientAllergiesResponseAllergiesItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Doctor => write!(f, "Doctor"),
            Self::Patient => write!(f, "Patient"),
            Self::PatientAgentGuardian => write!(f, "Patient Agent/Guardian"),
            Self::Pharmacist => write!(f, "Pharmacist"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

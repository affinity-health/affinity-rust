pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAccountResponseMembershipRole {
    Administrator,
    ClinicalReviewer,
    Developer,
    Operations,
    Owner,
    Viewer,
    ServiceKey,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAccountResponseMembershipRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Administrator => serializer.serialize_str("administrator"),
            Self::ClinicalReviewer => serializer.serialize_str("clinical_reviewer"),
            Self::Developer => serializer.serialize_str("developer"),
            Self::Operations => serializer.serialize_str("operations"),
            Self::Owner => serializer.serialize_str("owner"),
            Self::Viewer => serializer.serialize_str("viewer"),
            Self::ServiceKey => serializer.serialize_str("service_key"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAccountResponseMembershipRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "administrator" => Ok(Self::Administrator),
            "clinical_reviewer" => Ok(Self::ClinicalReviewer),
            "developer" => Ok(Self::Developer),
            "operations" => Ok(Self::Operations),
            "owner" => Ok(Self::Owner),
            "viewer" => Ok(Self::Viewer),
            "service_key" => Ok(Self::ServiceKey),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAccountResponseMembershipRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Administrator => write!(f, "administrator"),
            Self::ClinicalReviewer => write!(f, "clinical_reviewer"),
            Self::Developer => write!(f, "developer"),
            Self::Operations => write!(f, "operations"),
            Self::Owner => write!(f, "owner"),
            Self::Viewer => write!(f, "viewer"),
            Self::ServiceKey => write!(f, "service_key"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

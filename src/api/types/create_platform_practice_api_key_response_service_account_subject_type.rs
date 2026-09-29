pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType {
    Practice,
    InternalService,
    Pharmacy,
    Platform,
    User,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Practice => serializer.serialize_str("practice"),
            Self::InternalService => serializer.serialize_str("internal_service"),
            Self::Pharmacy => serializer.serialize_str("pharmacy"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::User => serializer.serialize_str("user"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "practice" => Ok(Self::Practice),
            "internal_service" => Ok(Self::InternalService),
            "pharmacy" => Ok(Self::Pharmacy),
            "platform" => Ok(Self::Platform),
            "user" => Ok(Self::User),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Practice => write!(f, "practice"),
            Self::InternalService => write!(f, "internal_service"),
            Self::Pharmacy => write!(f, "pharmacy"),
            Self::Platform => write!(f, "platform"),
            Self::User => write!(f, "user"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

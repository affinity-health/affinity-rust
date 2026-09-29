pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetPatientAllergiesResponseReviewStatus {
    NotReviewed,
    NoKnown,
    Recorded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetPatientAllergiesResponseReviewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotReviewed => serializer.serialize_str("not_reviewed"),
            Self::NoKnown => serializer.serialize_str("no_known"),
            Self::Recorded => serializer.serialize_str("recorded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetPatientAllergiesResponseReviewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_reviewed" => Ok(Self::NotReviewed),
            "no_known" => Ok(Self::NoKnown),
            "recorded" => Ok(Self::Recorded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetPatientAllergiesResponseReviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotReviewed => write!(f, "not_reviewed"),
            Self::NoKnown => write!(f, "no_known"),
            Self::Recorded => write!(f, "recorded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseDataItemPrescriptionsItemPatientSnapshotAllergyReviewStatus {
    NoKnown,
    NotReviewed,
    Recorded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOrdersResponseDataItemPrescriptionsItemPatientSnapshotAllergyReviewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NoKnown => serializer.serialize_str("no_known"),
            Self::NotReviewed => serializer.serialize_str("not_reviewed"),
            Self::Recorded => serializer.serialize_str("recorded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for ListOrdersResponseDataItemPrescriptionsItemPatientSnapshotAllergyReviewStatus
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "no_known" => Ok(Self::NoKnown),
            "not_reviewed" => Ok(Self::NotReviewed),
            "recorded" => Ok(Self::Recorded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for ListOrdersResponseDataItemPrescriptionsItemPatientSnapshotAllergyReviewStatus
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoKnown => write!(f, "no_known"),
            Self::NotReviewed => write!(f, "not_reviewed"),
            Self::Recorded => write!(f, "recorded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActOnOrderExceptionRequestAction {
    Acknowledge,
    AssignToMe,
    ContactPharmacy,
    RecordOutcome,
    Resolve,
    Retry,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ActOnOrderExceptionRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Acknowledge => serializer.serialize_str("acknowledge"),
            Self::AssignToMe => serializer.serialize_str("assign_to_me"),
            Self::ContactPharmacy => serializer.serialize_str("contact_pharmacy"),
            Self::RecordOutcome => serializer.serialize_str("record_outcome"),
            Self::Resolve => serializer.serialize_str("resolve"),
            Self::Retry => serializer.serialize_str("retry"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ActOnOrderExceptionRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "acknowledge" => Ok(Self::Acknowledge),
            "assign_to_me" => Ok(Self::AssignToMe),
            "contact_pharmacy" => Ok(Self::ContactPharmacy),
            "record_outcome" => Ok(Self::RecordOutcome),
            "resolve" => Ok(Self::Resolve),
            "retry" => Ok(Self::Retry),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ActOnOrderExceptionRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Acknowledge => write!(f, "acknowledge"),
            Self::AssignToMe => write!(f, "assign_to_me"),
            Self::ContactPharmacy => write!(f, "contact_pharmacy"),
            Self::RecordOutcome => write!(f, "record_outcome"),
            Self::Resolve => write!(f, "resolve"),
            Self::Retry => write!(f, "retry"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

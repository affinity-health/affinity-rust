pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvitePracticeTeamPersonResponseDelivery {
    Sent,
    AlreadyAccepted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvitePracticeTeamPersonResponseDelivery {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sent => serializer.serialize_str("sent"),
            Self::AlreadyAccepted => serializer.serialize_str("already_accepted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvitePracticeTeamPersonResponseDelivery {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sent" => Ok(Self::Sent),
            "already_accepted" => Ok(Self::AlreadyAccepted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvitePracticeTeamPersonResponseDelivery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::AlreadyAccepted => write!(f, "already_accepted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

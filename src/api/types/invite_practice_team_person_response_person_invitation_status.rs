pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvitePracticeTeamPersonResponsePersonInvitationStatus {
    Accepted,
    Declined,
    Pending,
    Expired,
    Revoked,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvitePracticeTeamPersonResponsePersonInvitationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Revoked => serializer.serialize_str("revoked"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvitePracticeTeamPersonResponsePersonInvitationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accepted" => Ok(Self::Accepted),
            "declined" => Ok(Self::Declined),
            "pending" => Ok(Self::Pending),
            "expired" => Ok(Self::Expired),
            "revoked" => Ok(Self::Revoked),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvitePracticeTeamPersonResponsePersonInvitationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "accepted"),
            Self::Declined => write!(f, "declined"),
            Self::Pending => write!(f, "pending"),
            Self::Expired => write!(f, "expired"),
            Self::Revoked => write!(f, "revoked"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

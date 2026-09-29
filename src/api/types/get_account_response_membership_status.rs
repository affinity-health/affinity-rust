pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetAccountResponseMembershipStatus {
    Active,
    Disabled,
    Invited,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetAccountResponseMembershipStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::Invited => serializer.serialize_str("invited"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetAccountResponseMembershipStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            "invited" => Ok(Self::Invited),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetAccountResponseMembershipStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Disabled => write!(f, "disabled"),
            Self::Invited => write!(f, "invited"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

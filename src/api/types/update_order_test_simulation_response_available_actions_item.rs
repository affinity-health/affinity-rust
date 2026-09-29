pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateOrderTestSimulationResponseAvailableActionsItem {
    Accept,
    Process,
    Ship,
    Deliver,
    Reject,
    ConfirmCancellation,
    DeclineCancellation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateOrderTestSimulationResponseAvailableActionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accept => serializer.serialize_str("accept"),
            Self::Process => serializer.serialize_str("process"),
            Self::Ship => serializer.serialize_str("ship"),
            Self::Deliver => serializer.serialize_str("deliver"),
            Self::Reject => serializer.serialize_str("reject"),
            Self::ConfirmCancellation => serializer.serialize_str("confirm_cancellation"),
            Self::DeclineCancellation => serializer.serialize_str("decline_cancellation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateOrderTestSimulationResponseAvailableActionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accept" => Ok(Self::Accept),
            "process" => Ok(Self::Process),
            "ship" => Ok(Self::Ship),
            "deliver" => Ok(Self::Deliver),
            "reject" => Ok(Self::Reject),
            "confirm_cancellation" => Ok(Self::ConfirmCancellation),
            "decline_cancellation" => Ok(Self::DeclineCancellation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateOrderTestSimulationResponseAvailableActionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accept => write!(f, "accept"),
            Self::Process => write!(f, "process"),
            Self::Ship => write!(f, "ship"),
            Self::Deliver => write!(f, "deliver"),
            Self::Reject => write!(f, "reject"),
            Self::ConfirmCancellation => write!(f, "confirm_cancellation"),
            Self::DeclineCancellation => write!(f, "decline_cancellation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

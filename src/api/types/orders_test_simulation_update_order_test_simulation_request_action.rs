pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateOrderTestSimulationRequestAction {
    Ship,
    Deliver,
    Accept,
    Process,
    Reject,
    ConfirmCancellation,
    DeclineCancellation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateOrderTestSimulationRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ship => serializer.serialize_str("ship"),
            Self::Deliver => serializer.serialize_str("deliver"),
            Self::Accept => serializer.serialize_str("accept"),
            Self::Process => serializer.serialize_str("process"),
            Self::Reject => serializer.serialize_str("reject"),
            Self::ConfirmCancellation => serializer.serialize_str("confirm_cancellation"),
            Self::DeclineCancellation => serializer.serialize_str("decline_cancellation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateOrderTestSimulationRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ship" => Ok(Self::Ship),
            "deliver" => Ok(Self::Deliver),
            "accept" => Ok(Self::Accept),
            "process" => Ok(Self::Process),
            "reject" => Ok(Self::Reject),
            "confirm_cancellation" => Ok(Self::ConfirmCancellation),
            "decline_cancellation" => Ok(Self::DeclineCancellation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateOrderTestSimulationRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ship => write!(f, "ship"),
            Self::Deliver => write!(f, "deliver"),
            Self::Accept => write!(f, "accept"),
            Self::Process => write!(f, "process"),
            Self::Reject => write!(f, "reject"),
            Self::ConfirmCancellation => write!(f, "confirm_cancellation"),
            Self::DeclineCancellation => write!(f, "decline_cancellation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

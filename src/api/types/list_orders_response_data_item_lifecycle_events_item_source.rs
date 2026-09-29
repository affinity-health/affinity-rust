pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseDataItemLifecycleEventsItemSource {
    Cancellation,
    Exception,
    Fulfillment,
    Integration,
    Shipment,
    Webhook,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOrdersResponseDataItemLifecycleEventsItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cancellation => serializer.serialize_str("cancellation"),
            Self::Exception => serializer.serialize_str("exception"),
            Self::Fulfillment => serializer.serialize_str("fulfillment"),
            Self::Integration => serializer.serialize_str("integration"),
            Self::Shipment => serializer.serialize_str("shipment"),
            Self::Webhook => serializer.serialize_str("webhook"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListOrdersResponseDataItemLifecycleEventsItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cancellation" => Ok(Self::Cancellation),
            "exception" => Ok(Self::Exception),
            "fulfillment" => Ok(Self::Fulfillment),
            "integration" => Ok(Self::Integration),
            "shipment" => Ok(Self::Shipment),
            "webhook" => Ok(Self::Webhook),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListOrdersResponseDataItemLifecycleEventsItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancellation => write!(f, "cancellation"),
            Self::Exception => write!(f, "exception"),
            Self::Fulfillment => write!(f, "fulfillment"),
            Self::Integration => write!(f, "integration"),
            Self::Shipment => write!(f, "shipment"),
            Self::Webhook => write!(f, "webhook"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

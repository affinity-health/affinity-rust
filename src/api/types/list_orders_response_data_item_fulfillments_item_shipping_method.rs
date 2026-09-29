pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseDataItemFulfillmentsItemShippingMethod {
    Standard,
    Expedited,
    Overnight,
    Pickup,
    LocalDelivery,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListOrdersResponseDataItemFulfillmentsItemShippingMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::Expedited => serializer.serialize_str("expedited"),
            Self::Overnight => serializer.serialize_str("overnight"),
            Self::Pickup => serializer.serialize_str("pickup"),
            Self::LocalDelivery => serializer.serialize_str("local_delivery"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListOrdersResponseDataItemFulfillmentsItemShippingMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "expedited" => Ok(Self::Expedited),
            "overnight" => Ok(Self::Overnight),
            "pickup" => Ok(Self::Pickup),
            "local_delivery" => Ok(Self::LocalDelivery),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListOrdersResponseDataItemFulfillmentsItemShippingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Expedited => write!(f, "expedited"),
            Self::Overnight => write!(f, "overnight"),
            Self::Pickup => write!(f, "pickup"),
            Self::LocalDelivery => write!(f, "local_delivery"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

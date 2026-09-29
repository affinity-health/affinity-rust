pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetOrderResponseFulfillmentsItemShipmentsItemStatus {
    LabelCreated,
    CarrierPossession,
    InTransit,
    OutForDelivery,
    Delivered,
    Delayed,
    DeliveryFailed,
    Returned,
    Voided,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetOrderResponseFulfillmentsItemShipmentsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LabelCreated => serializer.serialize_str("label_created"),
            Self::CarrierPossession => serializer.serialize_str("carrier_possession"),
            Self::InTransit => serializer.serialize_str("in_transit"),
            Self::OutForDelivery => serializer.serialize_str("out_for_delivery"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Delayed => serializer.serialize_str("delayed"),
            Self::DeliveryFailed => serializer.serialize_str("delivery_failed"),
            Self::Returned => serializer.serialize_str("returned"),
            Self::Voided => serializer.serialize_str("voided"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetOrderResponseFulfillmentsItemShipmentsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "label_created" => Ok(Self::LabelCreated),
            "carrier_possession" => Ok(Self::CarrierPossession),
            "in_transit" => Ok(Self::InTransit),
            "out_for_delivery" => Ok(Self::OutForDelivery),
            "delivered" => Ok(Self::Delivered),
            "delayed" => Ok(Self::Delayed),
            "delivery_failed" => Ok(Self::DeliveryFailed),
            "returned" => Ok(Self::Returned),
            "voided" => Ok(Self::Voided),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetOrderResponseFulfillmentsItemShipmentsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LabelCreated => write!(f, "label_created"),
            Self::CarrierPossession => write!(f, "carrier_possession"),
            Self::InTransit => write!(f, "in_transit"),
            Self::OutForDelivery => write!(f, "out_for_delivery"),
            Self::Delivered => write!(f, "delivered"),
            Self::Delayed => write!(f, "delayed"),
            Self::DeliveryFailed => write!(f, "delivery_failed"),
            Self::Returned => write!(f, "returned"),
            Self::Voided => write!(f, "voided"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

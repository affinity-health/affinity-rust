pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateWebhookEndpointRequestSubscribedEventsItem {
    WebhookEndpointTest,
    CancellationRequested,
    CancellationSent,
    CancellationConfirmed,
    CancellationRejected,
    CancellationFailed,
    CancellationTooLate,
    OrderCreated,
    OrderUpdated,
    OrderReviewRequested,
    OrderChangesRequested,
    OrderSigned,
    OrderRejected,
    OrderSubmitted,
    OrderAccepted,
    OrderProcessing,
    OrderShipped,
    OrderDelivered,
    OrderBlocked,
    OrderCancelled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateWebhookEndpointRequestSubscribedEventsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WebhookEndpointTest => serializer.serialize_str("webhook_endpoint.test"),
            Self::CancellationRequested => serializer.serialize_str("cancellation.requested"),
            Self::CancellationSent => serializer.serialize_str("cancellation.sent"),
            Self::CancellationConfirmed => serializer.serialize_str("cancellation.confirmed"),
            Self::CancellationRejected => serializer.serialize_str("cancellation.rejected"),
            Self::CancellationFailed => serializer.serialize_str("cancellation.failed"),
            Self::CancellationTooLate => serializer.serialize_str("cancellation.too_late"),
            Self::OrderCreated => serializer.serialize_str("order.created"),
            Self::OrderUpdated => serializer.serialize_str("order.updated"),
            Self::OrderReviewRequested => serializer.serialize_str("order.review_requested"),
            Self::OrderChangesRequested => serializer.serialize_str("order.changes_requested"),
            Self::OrderSigned => serializer.serialize_str("order.signed"),
            Self::OrderRejected => serializer.serialize_str("order.rejected"),
            Self::OrderSubmitted => serializer.serialize_str("order.submitted"),
            Self::OrderAccepted => serializer.serialize_str("order.accepted"),
            Self::OrderProcessing => serializer.serialize_str("order.processing"),
            Self::OrderShipped => serializer.serialize_str("order.shipped"),
            Self::OrderDelivered => serializer.serialize_str("order.delivered"),
            Self::OrderBlocked => serializer.serialize_str("order.blocked"),
            Self::OrderCancelled => serializer.serialize_str("order.cancelled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateWebhookEndpointRequestSubscribedEventsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "webhook_endpoint.test" => Ok(Self::WebhookEndpointTest),
            "cancellation.requested" => Ok(Self::CancellationRequested),
            "cancellation.sent" => Ok(Self::CancellationSent),
            "cancellation.confirmed" => Ok(Self::CancellationConfirmed),
            "cancellation.rejected" => Ok(Self::CancellationRejected),
            "cancellation.failed" => Ok(Self::CancellationFailed),
            "cancellation.too_late" => Ok(Self::CancellationTooLate),
            "order.created" => Ok(Self::OrderCreated),
            "order.updated" => Ok(Self::OrderUpdated),
            "order.review_requested" => Ok(Self::OrderReviewRequested),
            "order.changes_requested" => Ok(Self::OrderChangesRequested),
            "order.signed" => Ok(Self::OrderSigned),
            "order.rejected" => Ok(Self::OrderRejected),
            "order.submitted" => Ok(Self::OrderSubmitted),
            "order.accepted" => Ok(Self::OrderAccepted),
            "order.processing" => Ok(Self::OrderProcessing),
            "order.shipped" => Ok(Self::OrderShipped),
            "order.delivered" => Ok(Self::OrderDelivered),
            "order.blocked" => Ok(Self::OrderBlocked),
            "order.cancelled" => Ok(Self::OrderCancelled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateWebhookEndpointRequestSubscribedEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WebhookEndpointTest => write!(f, "webhook_endpoint.test"),
            Self::CancellationRequested => write!(f, "cancellation.requested"),
            Self::CancellationSent => write!(f, "cancellation.sent"),
            Self::CancellationConfirmed => write!(f, "cancellation.confirmed"),
            Self::CancellationRejected => write!(f, "cancellation.rejected"),
            Self::CancellationFailed => write!(f, "cancellation.failed"),
            Self::CancellationTooLate => write!(f, "cancellation.too_late"),
            Self::OrderCreated => write!(f, "order.created"),
            Self::OrderUpdated => write!(f, "order.updated"),
            Self::OrderReviewRequested => write!(f, "order.review_requested"),
            Self::OrderChangesRequested => write!(f, "order.changes_requested"),
            Self::OrderSigned => write!(f, "order.signed"),
            Self::OrderRejected => write!(f, "order.rejected"),
            Self::OrderSubmitted => write!(f, "order.submitted"),
            Self::OrderAccepted => write!(f, "order.accepted"),
            Self::OrderProcessing => write!(f, "order.processing"),
            Self::OrderShipped => write!(f, "order.shipped"),
            Self::OrderDelivered => write!(f, "order.delivered"),
            Self::OrderBlocked => write!(f, "order.blocked"),
            Self::OrderCancelled => write!(f, "order.cancelled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}

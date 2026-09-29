pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseFulfillmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(default)]
    pub cancellations: Vec<CancelOrderResponseFulfillmentsItemCancellationsItem>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "trackingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(rename = "trackingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_status: Option<String>,
    #[serde(rename = "shippedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipped_at: Option<String>,
    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    #[serde(rename = "estimatedDeliveryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_at: Option<String>,
    #[serde(default)]
    pub exceptions: Vec<CancelOrderResponseFulfillmentsItemExceptionsItem>,
    pub shipping: CancelOrderResponseFulfillmentsItemShipping,
    #[serde(default)]
    pub shipments: Vec<CancelOrderResponseFulfillmentsItemShipmentsItem>,
    #[serde(rename = "trackingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl CancelOrderResponseFulfillmentsItem {
    pub fn builder() -> CancelOrderResponseFulfillmentsItemBuilder {
        <CancelOrderResponseFulfillmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseFulfillmentsItemBuilder {
    carrier: Option<String>,
    cancellations: Option<Vec<CancelOrderResponseFulfillmentsItemCancellationsItem>>,
    pharmacy_id: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    prescription_id: Option<String>,
    status: Option<String>,
    tracking_number: Option<String>,
    tracking_status: Option<String>,
    shipped_at: Option<String>,
    delivered_at: Option<String>,
    estimated_delivery_at: Option<String>,
    exceptions: Option<Vec<CancelOrderResponseFulfillmentsItemExceptionsItem>>,
    shipping: Option<CancelOrderResponseFulfillmentsItemShipping>,
    shipments: Option<Vec<CancelOrderResponseFulfillmentsItemShipmentsItem>>,
    tracking_url: Option<String>,
    updated_at: Option<String>,
}

impl CancelOrderResponseFulfillmentsItemBuilder {
    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn cancellations(
        mut self,
        value: Vec<CancelOrderResponseFulfillmentsItemCancellationsItem>,
    ) -> Self {
        self.cancellations = Some(value);
        self
    }

    pub fn pharmacy_id(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    pub fn tracking_status(mut self, value: impl Into<String>) -> Self {
        self.tracking_status = Some(value.into());
        self
    }

    pub fn shipped_at(mut self, value: impl Into<String>) -> Self {
        self.shipped_at = Some(value.into());
        self
    }

    pub fn delivered_at(mut self, value: impl Into<String>) -> Self {
        self.delivered_at = Some(value.into());
        self
    }

    pub fn estimated_delivery_at(mut self, value: impl Into<String>) -> Self {
        self.estimated_delivery_at = Some(value.into());
        self
    }

    pub fn exceptions(
        mut self,
        value: Vec<CancelOrderResponseFulfillmentsItemExceptionsItem>,
    ) -> Self {
        self.exceptions = Some(value);
        self
    }

    pub fn shipping(mut self, value: CancelOrderResponseFulfillmentsItemShipping) -> Self {
        self.shipping = Some(value);
        self
    }

    pub fn shipments(
        mut self,
        value: Vec<CancelOrderResponseFulfillmentsItemShipmentsItem>,
    ) -> Self {
        self.shipments = Some(value);
        self
    }

    pub fn tracking_url(mut self, value: impl Into<String>) -> Self {
        self.tracking_url = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseFulfillmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cancellations`](CancelOrderResponseFulfillmentsItemBuilder::cancellations)
    /// - [`created_at`](CancelOrderResponseFulfillmentsItemBuilder::created_at)
    /// - [`id`](CancelOrderResponseFulfillmentsItemBuilder::id)
    /// - [`prescription_id`](CancelOrderResponseFulfillmentsItemBuilder::prescription_id)
    /// - [`status`](CancelOrderResponseFulfillmentsItemBuilder::status)
    /// - [`exceptions`](CancelOrderResponseFulfillmentsItemBuilder::exceptions)
    /// - [`shipping`](CancelOrderResponseFulfillmentsItemBuilder::shipping)
    /// - [`shipments`](CancelOrderResponseFulfillmentsItemBuilder::shipments)
    /// - [`updated_at`](CancelOrderResponseFulfillmentsItemBuilder::updated_at)
    pub fn build(self) -> Result<CancelOrderResponseFulfillmentsItem, BuildError> {
        Ok(CancelOrderResponseFulfillmentsItem {
            carrier: self.carrier,
            cancellations: self
                .cancellations
                .ok_or_else(|| BuildError::missing_field("cancellations"))?,
            pharmacy_id: self.pharmacy_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tracking_number: self.tracking_number,
            tracking_status: self.tracking_status,
            shipped_at: self.shipped_at,
            delivered_at: self.delivered_at,
            estimated_delivery_at: self.estimated_delivery_at,
            exceptions: self
                .exceptions
                .ok_or_else(|| BuildError::missing_field("exceptions"))?,
            shipping: self
                .shipping
                .ok_or_else(|| BuildError::missing_field("shipping"))?,
            shipments: self
                .shipments
                .ok_or_else(|| BuildError::missing_field("shipments"))?,
            tracking_url: self.tracking_url,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

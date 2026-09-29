pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemShipmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
    #[serde(rename = "estimatedDeliveryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_delivery_at: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "providerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_status: Option<String>,
    #[serde(rename = "replacedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_at: Option<String>,
    #[serde(rename = "replacesShipmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces_shipment_id: Option<String>,
    #[serde(rename = "shippedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipped_at: Option<String>,
    pub source: ListOrdersResponseDataItemFulfillmentsItemShipmentsItemSource,
    pub status: ListOrdersResponseDataItemFulfillmentsItemShipmentsItemStatus,
    #[serde(rename = "trackingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
    #[serde(rename = "trackingUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_url: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(rename = "voidedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemShipmentsItem {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder {
        <ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder {
    carrier: Option<String>,
    created_at: Option<String>,
    delivered_at: Option<String>,
    estimated_delivery_at: Option<String>,
    id: Option<String>,
    is_active: Option<bool>,
    provider_status: Option<String>,
    replaced_at: Option<String>,
    replaces_shipment_id: Option<String>,
    shipped_at: Option<String>,
    source: Option<ListOrdersResponseDataItemFulfillmentsItemShipmentsItemSource>,
    status: Option<ListOrdersResponseDataItemFulfillmentsItemShipmentsItemStatus>,
    tracking_number: Option<String>,
    tracking_url: Option<String>,
    updated_at: Option<String>,
    voided_at: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder {
    pub fn carrier(mut self, value: impl Into<String>) -> Self {
        self.carrier = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn provider_status(mut self, value: impl Into<String>) -> Self {
        self.provider_status = Some(value.into());
        self
    }

    pub fn replaced_at(mut self, value: impl Into<String>) -> Self {
        self.replaced_at = Some(value.into());
        self
    }

    pub fn replaces_shipment_id(mut self, value: impl Into<String>) -> Self {
        self.replaces_shipment_id = Some(value.into());
        self
    }

    pub fn shipped_at(mut self, value: impl Into<String>) -> Self {
        self.shipped_at = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemShipmentsItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemShipmentsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
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

    pub fn voided_at(mut self, value: impl Into<String>) -> Self {
        self.voided_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemShipmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::created_at)
    /// - [`id`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::id)
    /// - [`is_active`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::is_active)
    /// - [`source`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::source)
    /// - [`status`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::status)
    /// - [`updated_at`](ListOrdersResponseDataItemFulfillmentsItemShipmentsItemBuilder::updated_at)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemFulfillmentsItemShipmentsItem, BuildError> {
        Ok(ListOrdersResponseDataItemFulfillmentsItemShipmentsItem {
            carrier: self.carrier,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivered_at: self.delivered_at,
            estimated_delivery_at: self.estimated_delivery_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            provider_status: self.provider_status,
            replaced_at: self.replaced_at,
            replaces_shipment_id: self.replaces_shipment_id,
            shipped_at: self.shipped_at,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tracking_number: self.tracking_number,
            tracking_url: self.tracking_url,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            voided_at: self.voided_at,
        })
    }
}

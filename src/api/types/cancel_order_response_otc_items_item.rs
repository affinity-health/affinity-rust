pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseOtcItemsItem {
    #[serde(rename = "catalogItemId")]
    #[serde(default)]
    pub catalog_item_id: String,
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub quantity: i64,
    #[serde(rename = "unitPriceCents")]
    #[serde(default)]
    pub unit_price_cents: i64,
    #[serde(rename = "subtotalCents")]
    #[serde(default)]
    pub subtotal_cents: i64,
}

impl CancelOrderResponseOtcItemsItem {
    pub fn builder() -> CancelOrderResponseOtcItemsItemBuilder {
        <CancelOrderResponseOtcItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseOtcItemsItemBuilder {
    catalog_item_id: Option<String>,
    prescription_id: Option<String>,
    name: Option<String>,
    quantity: Option<i64>,
    unit_price_cents: Option<i64>,
    subtotal_cents: Option<i64>,
}

impl CancelOrderResponseOtcItemsItemBuilder {
    pub fn catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_item_id = Some(value.into());
        self
    }

    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn unit_price_cents(mut self, value: i64) -> Self {
        self.unit_price_cents = Some(value);
        self
    }

    pub fn subtotal_cents(mut self, value: i64) -> Self {
        self.subtotal_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseOtcItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_item_id`](CancelOrderResponseOtcItemsItemBuilder::catalog_item_id)
    /// - [`prescription_id`](CancelOrderResponseOtcItemsItemBuilder::prescription_id)
    /// - [`name`](CancelOrderResponseOtcItemsItemBuilder::name)
    /// - [`quantity`](CancelOrderResponseOtcItemsItemBuilder::quantity)
    /// - [`unit_price_cents`](CancelOrderResponseOtcItemsItemBuilder::unit_price_cents)
    /// - [`subtotal_cents`](CancelOrderResponseOtcItemsItemBuilder::subtotal_cents)
    pub fn build(self) -> Result<CancelOrderResponseOtcItemsItem, BuildError> {
        Ok(CancelOrderResponseOtcItemsItem {
            catalog_item_id: self
                .catalog_item_id
                .ok_or_else(|| BuildError::missing_field("catalog_item_id"))?,
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            unit_price_cents: self
                .unit_price_cents
                .ok_or_else(|| BuildError::missing_field("unit_price_cents"))?,
            subtotal_cents: self
                .subtotal_cents
                .ok_or_else(|| BuildError::missing_field("subtotal_cents"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreviewOrderResponsePrescriptionsItem {
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<PreviewOrderResponsePrescriptionsItemStructuredSig>,
    pub format: PreviewOrderResponsePrescriptionsItemFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<PreviewOrderResponsePrescriptionsItemQuantity>,
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i64>,
    #[serde(rename = "daysSupplySource")]
    pub days_supply_source: PreviewOrderResponsePrescriptionsItemDaysSupplySource,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "shippingOptions")]
    #[serde(default)]
    pub shipping_options: Vec<PreviewOrderResponsePrescriptionsItemShippingOptionsItem>,
    #[serde(rename = "shippingOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(rename = "medicationSubtotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_subtotal_cents: Option<i64>,
    #[serde(rename = "shippingAmountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount_cents: Option<i64>,
}

impl PreviewOrderResponsePrescriptionsItem {
    pub fn builder() -> PreviewOrderResponsePrescriptionsItemBuilder {
        <PreviewOrderResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponsePrescriptionsItemBuilder {
    medication_id: Option<String>,
    revision: Option<String>,
    directions: Option<String>,
    structured_sig: Option<PreviewOrderResponsePrescriptionsItemStructuredSig>,
    format: Option<PreviewOrderResponsePrescriptionsItemFormat>,
    quantity: Option<PreviewOrderResponsePrescriptionsItemQuantity>,
    days_supply: Option<i64>,
    days_supply_source: Option<PreviewOrderResponsePrescriptionsItemDaysSupplySource>,
    refills: Option<i64>,
    shipping_options: Option<Vec<PreviewOrderResponsePrescriptionsItemShippingOptionsItem>>,
    shipping_option_id: Option<String>,
    medication_subtotal_cents: Option<i64>,
    shipping_amount_cents: Option<i64>,
}

impl PreviewOrderResponsePrescriptionsItemBuilder {
    pub fn medication_id(mut self, value: impl Into<String>) -> Self {
        self.medication_id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn structured_sig(
        mut self,
        value: PreviewOrderResponsePrescriptionsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    pub fn format(mut self, value: PreviewOrderResponsePrescriptionsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn quantity(mut self, value: PreviewOrderResponsePrescriptionsItemQuantity) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn days_supply_source(
        mut self,
        value: PreviewOrderResponsePrescriptionsItemDaysSupplySource,
    ) -> Self {
        self.days_supply_source = Some(value);
        self
    }

    pub fn refills(mut self, value: i64) -> Self {
        self.refills = Some(value);
        self
    }

    pub fn shipping_options(
        mut self,
        value: Vec<PreviewOrderResponsePrescriptionsItemShippingOptionsItem>,
    ) -> Self {
        self.shipping_options = Some(value);
        self
    }

    pub fn shipping_option_id(mut self, value: impl Into<String>) -> Self {
        self.shipping_option_id = Some(value.into());
        self
    }

    pub fn medication_subtotal_cents(mut self, value: i64) -> Self {
        self.medication_subtotal_cents = Some(value);
        self
    }

    pub fn shipping_amount_cents(mut self, value: i64) -> Self {
        self.shipping_amount_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`medication_id`](PreviewOrderResponsePrescriptionsItemBuilder::medication_id)
    /// - [`revision`](PreviewOrderResponsePrescriptionsItemBuilder::revision)
    /// - [`directions`](PreviewOrderResponsePrescriptionsItemBuilder::directions)
    /// - [`format`](PreviewOrderResponsePrescriptionsItemBuilder::format)
    /// - [`days_supply_source`](PreviewOrderResponsePrescriptionsItemBuilder::days_supply_source)
    /// - [`refills`](PreviewOrderResponsePrescriptionsItemBuilder::refills)
    /// - [`shipping_options`](PreviewOrderResponsePrescriptionsItemBuilder::shipping_options)
    pub fn build(self) -> Result<PreviewOrderResponsePrescriptionsItem, BuildError> {
        Ok(PreviewOrderResponsePrescriptionsItem {
            medication_id: self
                .medication_id
                .ok_or_else(|| BuildError::missing_field("medication_id"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            structured_sig: self.structured_sig,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            quantity: self.quantity,
            days_supply: self.days_supply,
            days_supply_source: self
                .days_supply_source
                .ok_or_else(|| BuildError::missing_field("days_supply_source"))?,
            refills: self
                .refills
                .ok_or_else(|| BuildError::missing_field("refills"))?,
            shipping_options: self
                .shipping_options
                .ok_or_else(|| BuildError::missing_field("shipping_options"))?,
            shipping_option_id: self.shipping_option_id,
            medication_subtotal_cents: self.medication_subtotal_cents,
            shipping_amount_cents: self.shipping_amount_cents,
        })
    }
}

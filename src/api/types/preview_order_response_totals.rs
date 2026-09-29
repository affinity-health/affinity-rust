pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseTotals {
    pub currency: PreviewOrderResponseTotalsCurrency,
    #[serde(rename = "medicationSubtotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_subtotal_cents: Option<i64>,
    #[serde(rename = "supplySubtotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supply_subtotal_cents: Option<i64>,
    #[serde(rename = "shippingTotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_total_cents: Option<i64>,
    #[serde(rename = "estimatedTotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_total_cents: Option<i64>,
}

impl PreviewOrderResponseTotals {
    pub fn builder() -> PreviewOrderResponseTotalsBuilder {
        <PreviewOrderResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseTotalsBuilder {
    currency: Option<PreviewOrderResponseTotalsCurrency>,
    medication_subtotal_cents: Option<i64>,
    supply_subtotal_cents: Option<i64>,
    shipping_total_cents: Option<i64>,
    estimated_total_cents: Option<i64>,
}

impl PreviewOrderResponseTotalsBuilder {
    pub fn currency(mut self, value: PreviewOrderResponseTotalsCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn medication_subtotal_cents(mut self, value: i64) -> Self {
        self.medication_subtotal_cents = Some(value);
        self
    }

    pub fn supply_subtotal_cents(mut self, value: i64) -> Self {
        self.supply_subtotal_cents = Some(value);
        self
    }

    pub fn shipping_total_cents(mut self, value: i64) -> Self {
        self.shipping_total_cents = Some(value);
        self
    }

    pub fn estimated_total_cents(mut self, value: i64) -> Self {
        self.estimated_total_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](PreviewOrderResponseTotalsBuilder::currency)
    pub fn build(self) -> Result<PreviewOrderResponseTotals, BuildError> {
        Ok(PreviewOrderResponseTotals {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            medication_subtotal_cents: self.medication_subtotal_cents,
            supply_subtotal_cents: self.supply_subtotal_cents,
            shipping_total_cents: self.shipping_total_cents,
            estimated_total_cents: self.estimated_total_cents,
        })
    }
}

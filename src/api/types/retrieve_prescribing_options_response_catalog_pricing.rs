pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogPricing {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    pub basis: RetrievePrescribingOptionsResponseCatalogPricingBasis,
    pub currency: RetrievePrescribingOptionsResponseCatalogPricingCurrency,
    #[serde(rename = "medicationSubtotalCents")]
    #[serde(default)]
    pub medication_subtotal_cents: i64,
}

impl RetrievePrescribingOptionsResponseCatalogPricing {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogPricingBuilder {
        <RetrievePrescribingOptionsResponseCatalogPricingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogPricingBuilder {
    amount_cents: Option<i64>,
    basis: Option<RetrievePrescribingOptionsResponseCatalogPricingBasis>,
    currency: Option<RetrievePrescribingOptionsResponseCatalogPricingCurrency>,
    medication_subtotal_cents: Option<i64>,
}

impl RetrievePrescribingOptionsResponseCatalogPricingBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn basis(mut self, value: RetrievePrescribingOptionsResponseCatalogPricingBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn currency(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPricingCurrency,
    ) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn medication_subtotal_cents(mut self, value: i64) -> Self {
        self.medication_subtotal_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogPricing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](RetrievePrescribingOptionsResponseCatalogPricingBuilder::amount_cents)
    /// - [`basis`](RetrievePrescribingOptionsResponseCatalogPricingBuilder::basis)
    /// - [`currency`](RetrievePrescribingOptionsResponseCatalogPricingBuilder::currency)
    /// - [`medication_subtotal_cents`](RetrievePrescribingOptionsResponseCatalogPricingBuilder::medication_subtotal_cents)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogPricing, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogPricing {
            amount_cents: self
                .amount_cents
                .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            basis: self
                .basis
                .ok_or_else(|| BuildError::missing_field("basis"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            medication_subtotal_cents: self
                .medication_subtotal_cents
                .ok_or_else(|| BuildError::missing_field("medication_subtotal_cents"))?,
        })
    }
}

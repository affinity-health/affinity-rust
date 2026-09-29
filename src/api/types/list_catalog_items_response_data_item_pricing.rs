pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemPricing {
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
    pub basis: ListCatalogItemsResponseDataItemPricingBasis,
    pub currency: ListCatalogItemsResponseDataItemPricingCurrency,
    #[serde(rename = "medicationSubtotalCents")]
    #[serde(default)]
    pub medication_subtotal_cents: i64,
}

impl ListCatalogItemsResponseDataItemPricing {
    pub fn builder() -> ListCatalogItemsResponseDataItemPricingBuilder {
        <ListCatalogItemsResponseDataItemPricingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemPricingBuilder {
    amount_cents: Option<i64>,
    basis: Option<ListCatalogItemsResponseDataItemPricingBasis>,
    currency: Option<ListCatalogItemsResponseDataItemPricingCurrency>,
    medication_subtotal_cents: Option<i64>,
}

impl ListCatalogItemsResponseDataItemPricingBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn basis(mut self, value: ListCatalogItemsResponseDataItemPricingBasis) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn currency(mut self, value: ListCatalogItemsResponseDataItemPricingCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn medication_subtotal_cents(mut self, value: i64) -> Self {
        self.medication_subtotal_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemPricing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_cents`](ListCatalogItemsResponseDataItemPricingBuilder::amount_cents)
    /// - [`basis`](ListCatalogItemsResponseDataItemPricingBuilder::basis)
    /// - [`currency`](ListCatalogItemsResponseDataItemPricingBuilder::currency)
    /// - [`medication_subtotal_cents`](ListCatalogItemsResponseDataItemPricingBuilder::medication_subtotal_cents)
    pub fn build(self) -> Result<ListCatalogItemsResponseDataItemPricing, BuildError> {
        Ok(ListCatalogItemsResponseDataItemPricing {
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

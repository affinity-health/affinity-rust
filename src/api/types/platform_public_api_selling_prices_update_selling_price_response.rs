pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlatformPublicApiSellingPricesUpdateSellingPriceResponse {
    #[serde(rename = "amountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(default)]
    pub version: i64,
    pub currency: PlatformPublicApiSellingPricesUpdateSellingPriceResponseCurrency,
    pub basis: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasis,
    #[serde(rename = "purchaseAmountCents")]
    #[serde(default)]
    pub purchase_amount_cents: i64,
    #[serde(rename = "requiresReview")]
    #[serde(default)]
    pub requires_review: bool,
}

impl PlatformPublicApiSellingPricesUpdateSellingPriceResponse {
    pub fn builder() -> PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder {
        <PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder {
    amount_cents: Option<i64>,
    version: Option<i64>,
    currency: Option<PlatformPublicApiSellingPricesUpdateSellingPriceResponseCurrency>,
    basis: Option<PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasis>,
    purchase_amount_cents: Option<i64>,
    requires_review: Option<bool>,
}

impl PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder {
    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn currency(
        mut self,
        value: PlatformPublicApiSellingPricesUpdateSellingPriceResponseCurrency,
    ) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn basis(
        mut self,
        value: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasis,
    ) -> Self {
        self.basis = Some(value);
        self
    }

    pub fn purchase_amount_cents(mut self, value: i64) -> Self {
        self.purchase_amount_cents = Some(value);
        self
    }

    pub fn requires_review(mut self, value: bool) -> Self {
        self.requires_review = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesUpdateSellingPriceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder::version)
    /// - [`currency`](PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder::currency)
    /// - [`basis`](PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder::basis)
    /// - [`purchase_amount_cents`](PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder::purchase_amount_cents)
    /// - [`requires_review`](PlatformPublicApiSellingPricesUpdateSellingPriceResponseBuilder::requires_review)
    pub fn build(
        self,
    ) -> Result<PlatformPublicApiSellingPricesUpdateSellingPriceResponse, BuildError> {
        Ok(PlatformPublicApiSellingPricesUpdateSellingPriceResponse {
            amount_cents: self.amount_cents,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            basis: self
                .basis
                .ok_or_else(|| BuildError::missing_field("basis"))?,
            purchase_amount_cents: self
                .purchase_amount_cents
                .ok_or_else(|| BuildError::missing_field("purchase_amount_cents"))?,
            requires_review: self
                .requires_review
                .ok_or_else(|| BuildError::missing_field("requires_review"))?,
        })
    }
}

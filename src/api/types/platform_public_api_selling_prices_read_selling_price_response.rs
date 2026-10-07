pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlatformPublicApiSellingPricesReadSellingPriceResponse {
    #[serde(rename = "amountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(default)]
    pub version: i64,
    pub currency: PlatformPublicApiSellingPricesReadSellingPriceResponseCurrency,
    #[serde(rename = "affinityPriceCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_price_cents: Option<i64>,
    #[serde(rename = "affinityBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_basis: Option<PlatformPublicApiSellingPricesReadSellingPriceResponseAffinityBasis>,
    pub basis: PlatformPublicApiSellingPricesReadSellingPriceResponseBasis,
    #[serde(rename = "purchaseAmountCents")]
    #[serde(default)]
    pub purchase_amount_cents: i64,
    #[serde(rename = "requiresReview")]
    #[serde(default)]
    pub requires_review: bool,
}

impl PlatformPublicApiSellingPricesReadSellingPriceResponse {
    pub fn builder() -> PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder {
        <PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder {
    amount_cents: Option<i64>,
    version: Option<i64>,
    currency: Option<PlatformPublicApiSellingPricesReadSellingPriceResponseCurrency>,
    affinity_price_cents: Option<i64>,
    affinity_basis: Option<PlatformPublicApiSellingPricesReadSellingPriceResponseAffinityBasis>,
    basis: Option<PlatformPublicApiSellingPricesReadSellingPriceResponseBasis>,
    purchase_amount_cents: Option<i64>,
    requires_review: Option<bool>,
}

impl PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder {
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
        value: PlatformPublicApiSellingPricesReadSellingPriceResponseCurrency,
    ) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn affinity_price_cents(mut self, value: i64) -> Self {
        self.affinity_price_cents = Some(value);
        self
    }

    pub fn affinity_basis(
        mut self,
        value: PlatformPublicApiSellingPricesReadSellingPriceResponseAffinityBasis,
    ) -> Self {
        self.affinity_basis = Some(value);
        self
    }

    pub fn basis(
        mut self,
        value: PlatformPublicApiSellingPricesReadSellingPriceResponseBasis,
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

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadSellingPriceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder::version)
    /// - [`currency`](PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder::currency)
    /// - [`basis`](PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder::basis)
    /// - [`purchase_amount_cents`](PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder::purchase_amount_cents)
    /// - [`requires_review`](PlatformPublicApiSellingPricesReadSellingPriceResponseBuilder::requires_review)
    pub fn build(
        self,
    ) -> Result<PlatformPublicApiSellingPricesReadSellingPriceResponse, BuildError> {
        Ok(PlatformPublicApiSellingPricesReadSellingPriceResponse {
            amount_cents: self.amount_cents,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            affinity_price_cents: self.affinity_price_cents,
            affinity_basis: self.affinity_basis,
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

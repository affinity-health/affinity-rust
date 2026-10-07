pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponse {
    #[serde(rename = "amountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(default)]
    pub version: i64,
    pub currency: PlatformPublicApiSellingPricesReadPresentationPriceResponseCurrency,
    #[serde(rename = "affinityPriceCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_price_cents: Option<i64>,
    #[serde(rename = "affinityBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_basis:
        Option<PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis: Option<PlatformPublicApiSellingPricesReadPresentationPriceResponseBasis>,
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponse {
    pub fn builder() -> PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder {
        <PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder {
    amount_cents: Option<i64>,
    version: Option<i64>,
    currency: Option<PlatformPublicApiSellingPricesReadPresentationPriceResponseCurrency>,
    affinity_price_cents: Option<i64>,
    affinity_basis:
        Option<PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasis>,
    basis: Option<PlatformPublicApiSellingPricesReadPresentationPriceResponseBasis>,
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder {
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
        value: PlatformPublicApiSellingPricesReadPresentationPriceResponseCurrency,
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
        value: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasis,
    ) -> Self {
        self.affinity_basis = Some(value);
        self
    }

    pub fn basis(
        mut self,
        value: PlatformPublicApiSellingPricesReadPresentationPriceResponseBasis,
    ) -> Self {
        self.basis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadPresentationPriceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder::version)
    /// - [`currency`](PlatformPublicApiSellingPricesReadPresentationPriceResponseBuilder::currency)
    pub fn build(
        self,
    ) -> Result<PlatformPublicApiSellingPricesReadPresentationPriceResponse, BuildError> {
        Ok(
            PlatformPublicApiSellingPricesReadPresentationPriceResponse {
                amount_cents: self.amount_cents,
                version: self
                    .version
                    .ok_or_else(|| BuildError::missing_field("version"))?,
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                affinity_price_cents: self.affinity_price_cents,
                affinity_basis: self.affinity_basis,
                basis: self.basis,
            },
        )
    }
}

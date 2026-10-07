pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem
{
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
}

impl
    PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem
{
    pub fn builder() -> PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder{
        <PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder
{
    quantity: Option<String>,
    amount_cents: Option<i64>,
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder {
    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder::quantity)
    /// - [`amount_cents`](PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItemBuilder::amount_cents)
    pub fn build(self) -> Result<PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem, BuildError> {
        Ok(PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem {
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
            amount_cents: self.amount_cents.ok_or_else(|| BuildError::missing_field("amount_cents"))?,
        })
    }
}

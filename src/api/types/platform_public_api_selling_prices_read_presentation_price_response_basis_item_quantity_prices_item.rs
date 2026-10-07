pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItem {
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItem {
    pub fn builder(
    ) -> PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder
    {
        <PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder
{
    quantity: Option<String>,
    amount_cents: Option<i64>,
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder {
    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder::quantity)
    /// - [`amount_cents`](PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItemBuilder::amount_cents)
    pub fn build(
        self,
    ) -> Result<
        PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItem,
        BuildError,
    > {
        Ok(PlatformPublicApiSellingPricesReadPresentationPriceResponseBasisItemQuantityPricesItem {
            quantity: self.quantity.ok_or_else(|| BuildError::missing_field("quantity"))?,
            amount_cents: self.amount_cents.ok_or_else(|| BuildError::missing_field("amount_cents"))?,
        })
    }
}

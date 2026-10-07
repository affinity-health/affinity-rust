pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItem {
    #[serde(default)]
    pub quantity: String,
    #[serde(rename = "amountCents")]
    #[serde(default)]
    pub amount_cents: i64,
}

impl PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItem {
    pub fn builder(
    ) -> PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder
    {
        <PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder
{
    quantity: Option<String>,
    amount_cents: Option<i64>,
}

impl PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder {
    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder::quantity)
    /// - [`amount_cents`](PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItemBuilder::amount_cents)
    pub fn build(
        self,
    ) -> Result<
        PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItem,
        BuildError,
    > {
        Ok(
            PlatformPublicApiSellingPricesReadSellingPriceResponseBasisItemQuantityPricesItem {
                quantity: self
                    .quantity
                    .ok_or_else(|| BuildError::missing_field("quantity"))?,
                amount_cents: self
                    .amount_cents
                    .ok_or_else(|| BuildError::missing_field("amount_cents"))?,
            },
        )
    }
}

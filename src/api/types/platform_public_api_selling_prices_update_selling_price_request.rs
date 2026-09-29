pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformPublicApiSellingPricesUpdateSellingPriceRequest {
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
    #[serde(rename = "amountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_cents: Option<i64>,
    #[serde(rename = "baseVersion")]
    #[serde(default)]
    pub base_version: i64,
}

impl PlatformPublicApiSellingPricesUpdateSellingPriceRequest {
    pub fn builder() -> PlatformPublicApiSellingPricesUpdateSellingPriceRequestBuilder {
        <PlatformPublicApiSellingPricesUpdateSellingPriceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesUpdateSellingPriceRequestBuilder {
    practice_id: Option<String>,
    amount_cents: Option<i64>,
    base_version: Option<i64>,
}

impl PlatformPublicApiSellingPricesUpdateSellingPriceRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn amount_cents(mut self, value: i64) -> Self {
        self.amount_cents = Some(value);
        self
    }

    pub fn base_version(mut self, value: i64) -> Self {
        self.base_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesUpdateSellingPriceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_version`](PlatformPublicApiSellingPricesUpdateSellingPriceRequestBuilder::base_version)
    pub fn build(
        self,
    ) -> Result<PlatformPublicApiSellingPricesUpdateSellingPriceRequest, BuildError> {
        Ok(PlatformPublicApiSellingPricesUpdateSellingPriceRequest {
            practice_id: self.practice_id,
            amount_cents: self.amount_cents,
            base_version: self
                .base_version
                .ok_or_else(|| BuildError::missing_field("base_version"))?,
        })
    }
}

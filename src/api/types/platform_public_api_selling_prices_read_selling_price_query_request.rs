pub use crate::prelude::*;

/// Query parameters for platformPublicApiSellingPricesReadSellingPrice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformPublicApiSellingPricesReadSellingPriceQueryRequest {
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
}

impl PlatformPublicApiSellingPricesReadSellingPriceQueryRequest {
    pub fn builder() -> PlatformPublicApiSellingPricesReadSellingPriceQueryRequestBuilder {
        <PlatformPublicApiSellingPricesReadSellingPriceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformPublicApiSellingPricesReadSellingPriceQueryRequestBuilder {
    practice_id: Option<String>,
}

impl PlatformPublicApiSellingPricesReadSellingPriceQueryRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlatformPublicApiSellingPricesReadSellingPriceQueryRequest`].
    pub fn build(
        self,
    ) -> Result<PlatformPublicApiSellingPricesReadSellingPriceQueryRequest, BuildError> {
        Ok(PlatformPublicApiSellingPricesReadSellingPriceQueryRequest {
            practice_id: self.practice_id,
        })
    }
}

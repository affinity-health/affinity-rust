pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CatalogSellingPricesGetQueryRequest {
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
}

impl CatalogSellingPricesGetQueryRequest {
    pub fn builder() -> CatalogSellingPricesGetQueryRequestBuilder {
        <CatalogSellingPricesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CatalogSellingPricesGetQueryRequestBuilder {
    practice_id: Option<String>,
}

impl CatalogSellingPricesGetQueryRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CatalogSellingPricesGetQueryRequest`].
    pub fn build(self) -> Result<CatalogSellingPricesGetQueryRequest, BuildError> {
        Ok(CatalogSellingPricesGetQueryRequest {
            practice_id: self.practice_id,
        })
    }
}

pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CatalogPresentationPricesGetQueryRequest {
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
}

impl CatalogPresentationPricesGetQueryRequest {
    pub fn builder() -> CatalogPresentationPricesGetQueryRequestBuilder {
        <CatalogPresentationPricesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CatalogPresentationPricesGetQueryRequestBuilder {
    practice_id: Option<String>,
}

impl CatalogPresentationPricesGetQueryRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CatalogPresentationPricesGetQueryRequest`].
    pub fn build(self) -> Result<CatalogPresentationPricesGetQueryRequest, BuildError> {
        Ok(CatalogPresentationPricesGetQueryRequest {
            practice_id: self.practice_id,
        })
    }
}

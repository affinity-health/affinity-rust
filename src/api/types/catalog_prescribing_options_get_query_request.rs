pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CatalogPrescribingOptionsGetQueryRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
}

impl CatalogPrescribingOptionsGetQueryRequest {
    pub fn builder() -> CatalogPrescribingOptionsGetQueryRequestBuilder {
        <CatalogPrescribingOptionsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CatalogPrescribingOptionsGetQueryRequestBuilder {
    practice_id: Option<String>,
}

impl CatalogPrescribingOptionsGetQueryRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CatalogPrescribingOptionsGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](CatalogPrescribingOptionsGetQueryRequestBuilder::practice_id)
    pub fn build(self) -> Result<CatalogPrescribingOptionsGetQueryRequest, BuildError> {
        Ok(CatalogPrescribingOptionsGetQueryRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
        })
    }
}

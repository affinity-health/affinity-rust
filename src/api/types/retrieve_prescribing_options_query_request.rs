pub use crate::prelude::*;

/// Query parameters for retrievePrescribingOptions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsQueryRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
}

impl RetrievePrescribingOptionsQueryRequest {
    pub fn builder() -> RetrievePrescribingOptionsQueryRequestBuilder {
        <RetrievePrescribingOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsQueryRequestBuilder {
    practice_id: Option<String>,
}

impl RetrievePrescribingOptionsQueryRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](RetrievePrescribingOptionsQueryRequestBuilder::practice_id)
    pub fn build(self) -> Result<RetrievePrescribingOptionsQueryRequest, BuildError> {
        Ok(RetrievePrescribingOptionsQueryRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
        })
    }
}

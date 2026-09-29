pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseReviewResolvedBy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
}

impl CancelOrderResponseReviewResolvedBy {
    pub fn builder() -> CancelOrderResponseReviewResolvedByBuilder {
        <CancelOrderResponseReviewResolvedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseReviewResolvedByBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl CancelOrderResponseReviewResolvedByBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseReviewResolvedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CancelOrderResponseReviewResolvedByBuilder::id)
    /// - [`r#type`](CancelOrderResponseReviewResolvedByBuilder::r#type)
    pub fn build(self) -> Result<CancelOrderResponseReviewResolvedBy, BuildError> {
        Ok(CancelOrderResponseReviewResolvedBy {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponseReviewResolvedBy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
}

impl GetOrderResponseReviewResolvedBy {
    pub fn builder() -> GetOrderResponseReviewResolvedByBuilder {
        <GetOrderResponseReviewResolvedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseReviewResolvedByBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl GetOrderResponseReviewResolvedByBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseReviewResolvedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetOrderResponseReviewResolvedByBuilder::id)
    /// - [`r#type`](GetOrderResponseReviewResolvedByBuilder::r#type)
    pub fn build(self) -> Result<GetOrderResponseReviewResolvedBy, BuildError> {
        Ok(GetOrderResponseReviewResolvedBy {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

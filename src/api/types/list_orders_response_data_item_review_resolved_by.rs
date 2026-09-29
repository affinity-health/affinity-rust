pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemReviewResolvedBy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
}

impl ListOrdersResponseDataItemReviewResolvedBy {
    pub fn builder() -> ListOrdersResponseDataItemReviewResolvedByBuilder {
        <ListOrdersResponseDataItemReviewResolvedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemReviewResolvedByBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl ListOrdersResponseDataItemReviewResolvedByBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemReviewResolvedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListOrdersResponseDataItemReviewResolvedByBuilder::id)
    /// - [`r#type`](ListOrdersResponseDataItemReviewResolvedByBuilder::r#type)
    pub fn build(self) -> Result<ListOrdersResponseDataItemReviewResolvedBy, BuildError> {
        Ok(ListOrdersResponseDataItemReviewResolvedBy {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

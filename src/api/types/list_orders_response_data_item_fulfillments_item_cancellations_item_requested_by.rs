pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
}

impl ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder
    {
        <ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder::id)
    /// - [`r#type`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedByBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy, BuildError>
    {
        Ok(
            ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
            },
        )
    }
}

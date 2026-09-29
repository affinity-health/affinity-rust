pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponseFulfillmentsItemCancellationsItemRequestedBy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub r#type: String,
}

impl GetOrderResponseFulfillmentsItemCancellationsItemRequestedBy {
    pub fn builder() -> GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder {
        <GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder {
    id: Option<String>,
    r#type: Option<String>,
}

impl GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseFulfillmentsItemCancellationsItemRequestedBy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder::id)
    /// - [`r#type`](GetOrderResponseFulfillmentsItemCancellationsItemRequestedByBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<GetOrderResponseFulfillmentsItemCancellationsItemRequestedBy, BuildError> {
        Ok(
            GetOrderResponseFulfillmentsItemCancellationsItemRequestedBy {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
            },
        )
    }
}

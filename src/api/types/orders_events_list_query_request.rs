pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersEventsListQueryRequest {
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl OrdersEventsListQueryRequest {
    pub fn builder() -> OrdersEventsListQueryRequestBuilder {
        <OrdersEventsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersEventsListQueryRequestBuilder {
    ending_before: Option<String>,
    limit: Option<i64>,
    starting_after: Option<String>,
}

impl OrdersEventsListQueryRequestBuilder {
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersEventsListQueryRequest`].
    pub fn build(self) -> Result<OrdersEventsListQueryRequest, BuildError> {
        Ok(OrdersEventsListQueryRequest {
            ending_before: self.ending_before,
            limit: self.limit,
            starting_after: self.starting_after,
        })
    }
}

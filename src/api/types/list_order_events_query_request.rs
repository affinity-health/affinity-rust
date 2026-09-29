pub use crate::prelude::*;

/// Query parameters for listOrderEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrderEventsQueryRequest {
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl ListOrderEventsQueryRequest {
    pub fn builder() -> ListOrderEventsQueryRequestBuilder {
        <ListOrderEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrderEventsQueryRequestBuilder {
    ending_before: Option<String>,
    limit: Option<i64>,
    starting_after: Option<String>,
}

impl ListOrderEventsQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListOrderEventsQueryRequest`].
    pub fn build(self) -> Result<ListOrderEventsQueryRequest, BuildError> {
        Ok(ListOrderEventsQueryRequest {
            ending_before: self.ending_before,
            limit: self.limit,
            starting_after: self.starting_after,
        })
    }
}

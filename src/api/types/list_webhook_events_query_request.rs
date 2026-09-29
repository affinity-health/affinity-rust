pub use crate::prelude::*;

/// Query parameters for listWebhookEvents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhookEventsQueryRequest {
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListWebhookEventsRequestStatus>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl ListWebhookEventsQueryRequest {
    pub fn builder() -> ListWebhookEventsQueryRequestBuilder {
        <ListWebhookEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhookEventsQueryRequestBuilder {
    ending_before: Option<String>,
    limit: Option<i64>,
    status: Option<ListWebhookEventsRequestStatus>,
    starting_after: Option<String>,
}

impl ListWebhookEventsQueryRequestBuilder {
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn status(mut self, value: ListWebhookEventsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListWebhookEventsQueryRequest`].
    pub fn build(self) -> Result<ListWebhookEventsQueryRequest, BuildError> {
        Ok(ListWebhookEventsQueryRequest {
            ending_before: self.ending_before,
            limit: self.limit,
            status: self.status,
            starting_after: self.starting_after,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemCancellationsItem {
    #[serde(default)]
    pub attempts: i64,
    #[serde(rename = "confirmedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "providerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_status: Option<String>,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "requestedBy")]
    #[serde(default)]
    pub requested_by: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "sentAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    pub source: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemSource,
    pub status: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl ListOrdersResponseDataItemFulfillmentsItemCancellationsItem {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder {
        <ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder {
    attempts: Option<i64>,
    confirmed_at: Option<String>,
    created_at: Option<String>,
    error_code: Option<String>,
    error_message: Option<String>,
    id: Option<String>,
    provider_status: Option<String>,
    reason: Option<String>,
    requested_at: Option<String>,
    requested_by: Option<ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy>,
    resolved_at: Option<String>,
    sent_at: Option<String>,
    source: Option<ListOrdersResponseDataItemFulfillmentsItemCancellationsItemSource>,
    status: Option<ListOrdersResponseDataItemFulfillmentsItemCancellationsItemStatus>,
    updated_at: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn confirmed_at(mut self, value: impl Into<String>) -> Self {
        self.confirmed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error_code(mut self, value: impl Into<String>) -> Self {
        self.error_code = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn provider_status(mut self, value: impl Into<String>) -> Self {
        self.provider_status = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn requested_at(mut self, value: impl Into<String>) -> Self {
        self.requested_at = Some(value.into());
        self
    }

    pub fn requested_by(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemRequestedBy,
    ) -> Self {
        self.requested_by = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn sent_at(mut self, value: impl Into<String>) -> Self {
        self.sent_at = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemCancellationsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemCancellationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::attempts)
    /// - [`created_at`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::created_at)
    /// - [`id`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::id)
    /// - [`reason`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::reason)
    /// - [`requested_at`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::requested_at)
    /// - [`requested_by`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::requested_by)
    /// - [`source`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::source)
    /// - [`status`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::status)
    /// - [`updated_at`](ListOrdersResponseDataItemFulfillmentsItemCancellationsItemBuilder::updated_at)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemFulfillmentsItemCancellationsItem, BuildError> {
        Ok(
            ListOrdersResponseDataItemFulfillmentsItemCancellationsItem {
                attempts: self
                    .attempts
                    .ok_or_else(|| BuildError::missing_field("attempts"))?,
                confirmed_at: self.confirmed_at,
                created_at: self
                    .created_at
                    .ok_or_else(|| BuildError::missing_field("created_at"))?,
                error_code: self.error_code,
                error_message: self.error_message,
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                provider_status: self.provider_status,
                reason: self
                    .reason
                    .ok_or_else(|| BuildError::missing_field("reason"))?,
                requested_at: self
                    .requested_at
                    .ok_or_else(|| BuildError::missing_field("requested_at"))?,
                requested_by: self
                    .requested_by
                    .ok_or_else(|| BuildError::missing_field("requested_by"))?,
                resolved_at: self.resolved_at,
                sent_at: self.sent_at,
                source: self
                    .source
                    .ok_or_else(|| BuildError::missing_field("source"))?,
                status: self
                    .status
                    .ok_or_else(|| BuildError::missing_field("status"))?,
                updated_at: self
                    .updated_at
                    .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemExceptionsItem {
    #[serde(default)]
    pub actionable: bool,
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "dueAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(default)]
    pub retryable: bool,
    pub severity: ListOrdersResponseDataItemFulfillmentsItemExceptionsItemSeverity,
    pub status: ListOrdersResponseDataItemFulfillmentsItemExceptionsItemStatus,
    #[serde(default)]
    pub summary: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl ListOrdersResponseDataItemFulfillmentsItemExceptionsItem {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder {
        <ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder {
    actionable: Option<bool>,
    assigned_to: Option<ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo>,
    created_at: Option<String>,
    due_at: Option<String>,
    id: Option<String>,
    kind: Option<String>,
    resolution: Option<String>,
    resolved_at: Option<String>,
    retryable: Option<bool>,
    severity: Option<ListOrdersResponseDataItemFulfillmentsItemExceptionsItemSeverity>,
    status: Option<ListOrdersResponseDataItemFulfillmentsItemExceptionsItemStatus>,
    summary: Option<String>,
    updated_at: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder {
    pub fn actionable(mut self, value: bool) -> Self {
        self.actionable = Some(value);
        self
    }

    pub fn assigned_to(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo,
    ) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn due_at(mut self, value: impl Into<String>) -> Self {
        self.due_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn resolution(mut self, value: impl Into<String>) -> Self {
        self.resolution = Some(value.into());
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn severity(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemExceptionsItemSeverity,
    ) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: ListOrdersResponseDataItemFulfillmentsItemExceptionsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemExceptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actionable`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::actionable)
    /// - [`created_at`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::created_at)
    /// - [`id`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::id)
    /// - [`kind`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::kind)
    /// - [`retryable`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::retryable)
    /// - [`severity`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::severity)
    /// - [`status`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::status)
    /// - [`summary`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::summary)
    /// - [`updated_at`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemBuilder::updated_at)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemFulfillmentsItemExceptionsItem, BuildError> {
        Ok(ListOrdersResponseDataItemFulfillmentsItemExceptionsItem {
            actionable: self
                .actionable
                .ok_or_else(|| BuildError::missing_field("actionable"))?,
            assigned_to: self.assigned_to,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            due_at: self.due_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            resolution: self.resolution,
            resolved_at: self.resolved_at,
            retryable: self
                .retryable
                .ok_or_else(|| BuildError::missing_field("retryable"))?,
            severity: self
                .severity
                .ok_or_else(|| BuildError::missing_field("severity"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

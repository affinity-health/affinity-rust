pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseFulfillmentsItemExceptionsItem {
    #[serde(default)]
    pub actionable: bool,
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo>,
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
    pub severity: CancelOrderResponseFulfillmentsItemExceptionsItemSeverity,
    pub status: CancelOrderResponseFulfillmentsItemExceptionsItemStatus,
    #[serde(default)]
    pub summary: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl CancelOrderResponseFulfillmentsItemExceptionsItem {
    pub fn builder() -> CancelOrderResponseFulfillmentsItemExceptionsItemBuilder {
        <CancelOrderResponseFulfillmentsItemExceptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseFulfillmentsItemExceptionsItemBuilder {
    actionable: Option<bool>,
    assigned_to: Option<CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo>,
    created_at: Option<String>,
    due_at: Option<String>,
    id: Option<String>,
    kind: Option<String>,
    resolution: Option<String>,
    resolved_at: Option<String>,
    retryable: Option<bool>,
    severity: Option<CancelOrderResponseFulfillmentsItemExceptionsItemSeverity>,
    status: Option<CancelOrderResponseFulfillmentsItemExceptionsItemStatus>,
    summary: Option<String>,
    updated_at: Option<String>,
}

impl CancelOrderResponseFulfillmentsItemExceptionsItemBuilder {
    pub fn actionable(mut self, value: bool) -> Self {
        self.actionable = Some(value);
        self
    }

    pub fn assigned_to(
        mut self,
        value: CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo,
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
        value: CancelOrderResponseFulfillmentsItemExceptionsItemSeverity,
    ) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: CancelOrderResponseFulfillmentsItemExceptionsItemStatus,
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

    /// Consumes the builder and constructs a [`CancelOrderResponseFulfillmentsItemExceptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actionable`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::actionable)
    /// - [`created_at`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::created_at)
    /// - [`id`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::id)
    /// - [`kind`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::kind)
    /// - [`retryable`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::retryable)
    /// - [`severity`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::severity)
    /// - [`status`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::status)
    /// - [`summary`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::summary)
    /// - [`updated_at`](CancelOrderResponseFulfillmentsItemExceptionsItemBuilder::updated_at)
    pub fn build(self) -> Result<CancelOrderResponseFulfillmentsItemExceptionsItem, BuildError> {
        Ok(CancelOrderResponseFulfillmentsItemExceptionsItem {
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

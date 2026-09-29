pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseReview {
    pub status: CancelOrderResponseReviewStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "requestedAt")]
    #[serde(default)]
    pub requested_at: String,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "canceledAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<String>,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resolvedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<CancelOrderResponseReviewResolvedBy>,
    #[serde(rename = "providerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
}

impl CancelOrderResponseReview {
    pub fn builder() -> CancelOrderResponseReviewBuilder {
        <CancelOrderResponseReviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseReviewBuilder {
    status: Option<CancelOrderResponseReviewStatus>,
    reason: Option<String>,
    requested_at: Option<String>,
    completed_at: Option<String>,
    canceled_at: Option<String>,
    resolved_at: Option<String>,
    resolved_by: Option<CancelOrderResponseReviewResolvedBy>,
    provider_id: Option<String>,
}

impl CancelOrderResponseReviewBuilder {
    pub fn status(mut self, value: CancelOrderResponseReviewStatus) -> Self {
        self.status = Some(value);
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

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn canceled_at(mut self, value: impl Into<String>) -> Self {
        self.canceled_at = Some(value.into());
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn resolved_by(mut self, value: CancelOrderResponseReviewResolvedBy) -> Self {
        self.resolved_by = Some(value);
        self
    }

    pub fn provider_id(mut self, value: impl Into<String>) -> Self {
        self.provider_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseReview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CancelOrderResponseReviewBuilder::status)
    /// - [`requested_at`](CancelOrderResponseReviewBuilder::requested_at)
    pub fn build(self) -> Result<CancelOrderResponseReview, BuildError> {
        Ok(CancelOrderResponseReview {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            reason: self.reason,
            requested_at: self
                .requested_at
                .ok_or_else(|| BuildError::missing_field("requested_at"))?,
            completed_at: self.completed_at,
            canceled_at: self.canceled_at,
            resolved_at: self.resolved_at,
            resolved_by: self.resolved_by,
            provider_id: self.provider_id,
        })
    }
}

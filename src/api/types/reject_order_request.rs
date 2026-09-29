pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectOrderRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<RejectOrderRequestPrescriber>,
    #[serde(default)]
    pub reason: String,
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(rename = "expectedRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<String>,
    #[serde(rename = "expectedVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_versions: Option<Vec<RejectOrderRequestExpectedVersionsItem>>,
}

impl RejectOrderRequest {
    pub fn builder() -> RejectOrderRequestBuilder {
        <RejectOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectOrderRequestBuilder {
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<RejectOrderRequestPrescriber>,
    reason: Option<String>,
    expected_revision: Option<String>,
    expected_versions: Option<Vec<RejectOrderRequestExpectedVersionsItem>>,
}

impl RejectOrderRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: RejectOrderRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: impl Into<String>) -> Self {
        self.expected_revision = Some(value.into());
        self
    }

    pub fn expected_versions(mut self, value: Vec<RejectOrderRequestExpectedVersionsItem>) -> Self {
        self.expected_versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RejectOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](RejectOrderRequestBuilder::practice_id)
    /// - [`reason`](RejectOrderRequestBuilder::reason)
    pub fn build(self) -> Result<RejectOrderRequest, BuildError> {
        Ok(RejectOrderRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            expected_revision: self.expected_revision,
            expected_versions: self.expected_versions,
        })
    }
}

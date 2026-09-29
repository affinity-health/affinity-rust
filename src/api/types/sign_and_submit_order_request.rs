pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignAndSubmitOrderRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<SignAndSubmitOrderRequestPrescriber>,
    #[serde(rename = "signatureAttestation")]
    #[serde(default)]
    pub signature_attestation: bool,
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(rename = "expectedRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<String>,
    #[serde(rename = "expectedVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_versions: Option<Vec<SignAndSubmitOrderRequestExpectedVersionsItem>>,
}

impl SignAndSubmitOrderRequest {
    pub fn builder() -> SignAndSubmitOrderRequestBuilder {
        <SignAndSubmitOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderRequestBuilder {
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<SignAndSubmitOrderRequestPrescriber>,
    signature_attestation: Option<bool>,
    expected_revision: Option<String>,
    expected_versions: Option<Vec<SignAndSubmitOrderRequestExpectedVersionsItem>>,
}

impl SignAndSubmitOrderRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: SignAndSubmitOrderRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    pub fn signature_attestation(mut self, value: bool) -> Self {
        self.signature_attestation = Some(value);
        self
    }

    pub fn expected_revision(mut self, value: impl Into<String>) -> Self {
        self.expected_revision = Some(value.into());
        self
    }

    pub fn expected_versions(
        mut self,
        value: Vec<SignAndSubmitOrderRequestExpectedVersionsItem>,
    ) -> Self {
        self.expected_versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](SignAndSubmitOrderRequestBuilder::practice_id)
    /// - [`signature_attestation`](SignAndSubmitOrderRequestBuilder::signature_attestation)
    pub fn build(self) -> Result<SignAndSubmitOrderRequest, BuildError> {
        Ok(SignAndSubmitOrderRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
            signature_attestation: self
                .signature_attestation
                .ok_or_else(|| BuildError::missing_field("signature_attestation"))?,
            expected_revision: self.expected_revision,
            expected_versions: self.expected_versions,
        })
    }
}

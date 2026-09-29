pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateOrderPrescriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<UpdateOrderPrescriptionRequestMetadataValue>>>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(rename = "expectedRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision: Option<String>,
    #[serde(rename = "expectedVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_versions: Option<Vec<UpdateOrderPrescriptionRequestExpectedVersionsItem>>,
    pub prescription: UpdateOrderPrescriptionRequestPrescription,
}

impl UpdateOrderPrescriptionRequest {
    pub fn builder() -> UpdateOrderPrescriptionRequestBuilder {
        <UpdateOrderPrescriptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestBuilder {
    metadata: Option<HashMap<String, Option<UpdateOrderPrescriptionRequestMetadataValue>>>,
    practice_id: Option<String>,
    expected_revision: Option<String>,
    expected_versions: Option<Vec<UpdateOrderPrescriptionRequestExpectedVersionsItem>>,
    prescription: Option<UpdateOrderPrescriptionRequestPrescription>,
}

impl UpdateOrderPrescriptionRequestBuilder {
    pub fn metadata(
        mut self,
        value: HashMap<String, Option<UpdateOrderPrescriptionRequestMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: impl Into<String>) -> Self {
        self.expected_revision = Some(value.into());
        self
    }

    pub fn expected_versions(
        mut self,
        value: Vec<UpdateOrderPrescriptionRequestExpectedVersionsItem>,
    ) -> Self {
        self.expected_versions = Some(value);
        self
    }

    pub fn prescription(mut self, value: UpdateOrderPrescriptionRequestPrescription) -> Self {
        self.prescription = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](UpdateOrderPrescriptionRequestBuilder::practice_id)
    /// - [`prescription`](UpdateOrderPrescriptionRequestBuilder::prescription)
    pub fn build(self) -> Result<UpdateOrderPrescriptionRequest, BuildError> {
        Ok(UpdateOrderPrescriptionRequest {
            metadata: self.metadata,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            expected_revision: self.expected_revision,
            expected_versions: self.expected_versions,
            prescription: self
                .prescription
                .ok_or_else(|| BuildError::missing_field("prescription"))?,
        })
    }
}

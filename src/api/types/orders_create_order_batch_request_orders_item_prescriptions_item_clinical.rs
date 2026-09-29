pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses:
        Option<Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations:
        Option<Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem>>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalBuilder {
        <CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalBuilder {
    compounding_reason:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason>,
    medication_review_status:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem>>,
    observations:
        Option<Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem>>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosisReviewStatus,
    ) -> Self {
        self.diagnosis_review_status = Some(value);
        self
    }

    pub fn current_medications(mut self, value: Vec<String>) -> Self {
        self.current_medications = Some(value);
        self
    }

    pub fn diagnoses(
        mut self,
        value: Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical`].
    pub fn build(
        self,
    ) -> Result<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical, BuildError> {
        Ok(CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

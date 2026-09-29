pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderRequestPrescriptionsItemClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason: Option<CreateOrderRequestPrescriptionsItemClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<CreateOrderRequestPrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<CreateOrderRequestPrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<CreateOrderRequestPrescriptionsItemClinicalObservationsItem>>,
}

impl CreateOrderRequestPrescriptionsItemClinical {
    pub fn builder() -> CreateOrderRequestPrescriptionsItemClinicalBuilder {
        <CreateOrderRequestPrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriptionsItemClinicalBuilder {
    compounding_reason: Option<CreateOrderRequestPrescriptionsItemClinicalCompoundingReason>,
    medication_review_status:
        Option<CreateOrderRequestPrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<CreateOrderRequestPrescriptionsItemClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem>>,
    observations: Option<Vec<CreateOrderRequestPrescriptionsItemClinicalObservationsItem>>,
}

impl CreateOrderRequestPrescriptionsItemClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: CreateOrderRequestPrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: CreateOrderRequestPrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: CreateOrderRequestPrescriptionsItemClinicalDiagnosisReviewStatus,
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
        value: Vec<CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<CreateOrderRequestPrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriptionsItemClinical`].
    pub fn build(self) -> Result<CreateOrderRequestPrescriptionsItemClinical, BuildError> {
        Ok(CreateOrderRequestPrescriptionsItemClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

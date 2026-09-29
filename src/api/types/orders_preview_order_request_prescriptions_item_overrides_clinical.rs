pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations:
        Option<Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem>>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinical {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesClinicalBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalBuilder {
    compounding_reason:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason>,
    medication_review_status:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem>>,
    observations:
        Option<Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem>>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosisReviewStatus,
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
        value: Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesClinical`].
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesClinical, BuildError> {
        Ok(PreviewOrderRequestPrescriptionsItemOverridesClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses:
        Option<Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations:
        Option<Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem>>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinical {
    pub fn builder() -> PreviewOrderResponseOrderInputPrescriptionsItemClinicalBuilder {
        <PreviewOrderResponseOrderInputPrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalBuilder {
    compounding_reason:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason>,
    medication_review_status:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem>>,
    observations:
        Option<Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem>>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosisReviewStatus,
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
        value: Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriptionsItemClinical`].
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPrescriptionsItemClinical, BuildError> {
        Ok(PreviewOrderResponseOrderInputPrescriptionsItemClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

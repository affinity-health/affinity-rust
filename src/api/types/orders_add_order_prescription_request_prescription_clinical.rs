pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddOrderPrescriptionRequestPrescriptionClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<AddOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem>>,
}

impl AddOrderPrescriptionRequestPrescriptionClinical {
    pub fn builder() -> AddOrderPrescriptionRequestPrescriptionClinicalBuilder {
        <AddOrderPrescriptionRequestPrescriptionClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalBuilder {
    compounding_reason: Option<AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason>,
    medication_review_status:
        Option<AddOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>>,
    observations: Option<Vec<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem>>,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: AddOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: AddOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus,
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
        value: Vec<AddOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionRequestPrescriptionClinical`].
    pub fn build(self) -> Result<AddOrderPrescriptionRequestPrescriptionClinical, BuildError> {
        Ok(AddOrderPrescriptionRequestPrescriptionClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

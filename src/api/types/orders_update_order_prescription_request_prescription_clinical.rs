pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinical {
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus>,
    #[serde(rename = "currentMedications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_medications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations:
        Option<Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem>>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinical {
    pub fn builder() -> UpdateOrderPrescriptionRequestPrescriptionClinicalBuilder {
        <UpdateOrderPrescriptionRequestPrescriptionClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalBuilder {
    compounding_reason: Option<UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason>,
    medication_review_status:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus>,
    current_medications: Option<Vec<String>>,
    diagnoses: Option<Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>>,
    observations: Option<Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem>>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalBuilder {
    pub fn compounding_reason(
        mut self,
        value: UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: UpdateOrderPrescriptionRequestPrescriptionClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosisReviewStatus,
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
        value: Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalDiagnosesItem>,
    ) -> Self {
        self.diagnoses = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestPrescriptionClinical`].
    pub fn build(self) -> Result<UpdateOrderPrescriptionRequestPrescriptionClinical, BuildError> {
        Ok(UpdateOrderPrescriptionRequestPrescriptionClinical {
            compounding_reason: self.compounding_reason,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            current_medications: self.current_medications,
            diagnoses: self.diagnoses,
            observations: self.observations,
        })
    }
}

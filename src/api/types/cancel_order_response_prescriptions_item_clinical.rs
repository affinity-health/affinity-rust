pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CancelOrderResponsePrescriptionsItemClinical {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergies: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItem>>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<CancelOrderResponsePrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<CancelOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalConditionsItem>>,
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason: Option<CancelOrderResponsePrescriptionsItemClinicalCompoundingReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medications: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalMedicationsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalObservationsItem>>,
}

impl CancelOrderResponsePrescriptionsItemClinical {
    pub fn builder() -> CancelOrderResponsePrescriptionsItemClinicalBuilder {
        <CancelOrderResponsePrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponsePrescriptionsItemClinicalBuilder {
    allergies: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItem>>,
    medication_review_status:
        Option<CancelOrderResponsePrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<CancelOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus>,
    conditions: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalConditionsItem>>,
    compounding_reason: Option<CancelOrderResponsePrescriptionsItemClinicalCompoundingReason>,
    medications: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalMedicationsItem>>,
    observations: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalObservationsItem>>,
}

impl CancelOrderResponsePrescriptionsItemClinicalBuilder {
    pub fn allergies(
        mut self,
        value: Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItem>,
    ) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: CancelOrderResponsePrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: CancelOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus,
    ) -> Self {
        self.diagnosis_review_status = Some(value);
        self
    }

    pub fn conditions(
        mut self,
        value: Vec<CancelOrderResponsePrescriptionsItemClinicalConditionsItem>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn compounding_reason(
        mut self,
        value: CancelOrderResponsePrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medications(
        mut self,
        value: Vec<CancelOrderResponsePrescriptionsItemClinicalMedicationsItem>,
    ) -> Self {
        self.medications = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<CancelOrderResponsePrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponsePrescriptionsItemClinical`].
    pub fn build(self) -> Result<CancelOrderResponsePrescriptionsItemClinical, BuildError> {
        Ok(CancelOrderResponsePrescriptionsItemClinical {
            allergies: self.allergies,
            medication_review_status: self.medication_review_status,
            diagnosis_review_status: self.diagnosis_review_status,
            conditions: self.conditions,
            compounding_reason: self.compounding_reason,
            medications: self.medications,
            observations: self.observations,
        })
    }
}

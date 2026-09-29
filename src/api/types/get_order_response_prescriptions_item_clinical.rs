pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetOrderResponsePrescriptionsItemClinical {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergies: Option<Vec<GetOrderResponsePrescriptionsItemClinicalAllergiesItem>>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<GetOrderResponsePrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<GetOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GetOrderResponsePrescriptionsItemClinicalConditionsItem>>,
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason: Option<GetOrderResponsePrescriptionsItemClinicalCompoundingReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medications: Option<Vec<GetOrderResponsePrescriptionsItemClinicalMedicationsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations: Option<Vec<GetOrderResponsePrescriptionsItemClinicalObservationsItem>>,
}

impl GetOrderResponsePrescriptionsItemClinical {
    pub fn builder() -> GetOrderResponsePrescriptionsItemClinicalBuilder {
        <GetOrderResponsePrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemClinicalBuilder {
    allergies: Option<Vec<GetOrderResponsePrescriptionsItemClinicalAllergiesItem>>,
    medication_review_status:
        Option<GetOrderResponsePrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status: Option<GetOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus>,
    conditions: Option<Vec<GetOrderResponsePrescriptionsItemClinicalConditionsItem>>,
    compounding_reason: Option<GetOrderResponsePrescriptionsItemClinicalCompoundingReason>,
    medications: Option<Vec<GetOrderResponsePrescriptionsItemClinicalMedicationsItem>>,
    observations: Option<Vec<GetOrderResponsePrescriptionsItemClinicalObservationsItem>>,
}

impl GetOrderResponsePrescriptionsItemClinicalBuilder {
    pub fn allergies(
        mut self,
        value: Vec<GetOrderResponsePrescriptionsItemClinicalAllergiesItem>,
    ) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: GetOrderResponsePrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: GetOrderResponsePrescriptionsItemClinicalDiagnosisReviewStatus,
    ) -> Self {
        self.diagnosis_review_status = Some(value);
        self
    }

    pub fn conditions(
        mut self,
        value: Vec<GetOrderResponsePrescriptionsItemClinicalConditionsItem>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn compounding_reason(
        mut self,
        value: GetOrderResponsePrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medications(
        mut self,
        value: Vec<GetOrderResponsePrescriptionsItemClinicalMedicationsItem>,
    ) -> Self {
        self.medications = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<GetOrderResponsePrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemClinical`].
    pub fn build(self) -> Result<GetOrderResponsePrescriptionsItemClinical, BuildError> {
        Ok(GetOrderResponsePrescriptionsItemClinical {
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

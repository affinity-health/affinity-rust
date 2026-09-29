pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinical {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergies: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalAllergiesItem>>,
    #[serde(rename = "medicationReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review_status:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationReviewStatus>,
    #[serde(rename = "diagnosisReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review_status:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalDiagnosisReviewStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalConditionsItem>>,
    #[serde(rename = "compoundingReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medications:
        Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observations:
        Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem>>,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinical {
    pub fn builder() -> ListOrdersResponseDataItemPrescriptionsItemClinicalBuilder {
        <ListOrdersResponseDataItemPrescriptionsItemClinicalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalBuilder {
    allergies: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalAllergiesItem>>,
    medication_review_status:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationReviewStatus>,
    diagnosis_review_status:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalDiagnosisReviewStatus>,
    conditions: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalConditionsItem>>,
    compounding_reason:
        Option<ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason>,
    medications: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem>>,
    observations: Option<Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem>>,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalBuilder {
    pub fn allergies(
        mut self,
        value: Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalAllergiesItem>,
    ) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn medication_review_status(
        mut self,
        value: ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationReviewStatus,
    ) -> Self {
        self.medication_review_status = Some(value);
        self
    }

    pub fn diagnosis_review_status(
        mut self,
        value: ListOrdersResponseDataItemPrescriptionsItemClinicalDiagnosisReviewStatus,
    ) -> Self {
        self.diagnosis_review_status = Some(value);
        self
    }

    pub fn conditions(
        mut self,
        value: Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalConditionsItem>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn compounding_reason(
        mut self,
        value: ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn medications(
        mut self,
        value: Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem>,
    ) -> Self {
        self.medications = Some(value);
        self
    }

    pub fn observations(
        mut self,
        value: Vec<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem>,
    ) -> Self {
        self.observations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemPrescriptionsItemClinical`].
    pub fn build(self) -> Result<ListOrdersResponseDataItemPrescriptionsItemClinical, BuildError> {
        Ok(ListOrdersResponseDataItemPrescriptionsItemClinical {
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

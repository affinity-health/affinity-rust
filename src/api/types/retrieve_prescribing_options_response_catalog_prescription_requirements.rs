pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements {
    #[serde(rename = "allowedDaysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_days_supply: Option<Vec<i64>>,
    #[serde(rename = "allowedQuantities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_quantities: Option<Vec<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem>>,
    #[serde(rename = "allowedReasonCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_reason_categories: Option<Vec<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedReasonCategoriesItem>>,
    #[serde(rename = "reasonCategoryLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_category_labels: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "compoundingReason")]
    pub compounding_reason: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReason,
    #[serde(rename = "compoundingReasonContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason_context: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReasonContext>,
    #[serde(rename = "controlledSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled_schedule: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule>,
    #[serde(rename = "defaultDaysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_days_supply: Option<i64>,
    #[serde(rename = "defaultQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quantity: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity>,
    #[serde(rename = "quantityIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_increment: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement>,
    #[serde(rename = "defaultSigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sigs: Option<Vec<String>>,
    pub diagnosis: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosis,
    #[serde(rename = "medicationReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsMedicationReview>,
    #[serde(rename = "diagnosisReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosisReview>,
    #[serde(rename = "maxRefills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_refills: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    #[serde(rename = "pharmacyNotes")]
    pub pharmacy_notes: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsPharmacyNotes,
    pub refills: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsRefills,
    pub substitution: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder {
        <RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder {
    allowed_days_supply: Option<Vec<i64>>,
    allowed_quantities: Option<Vec<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem>>,
    allowed_reason_categories: Option<Vec<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedReasonCategoriesItem>>,
    reason_category_labels: Option<HashMap<String, Option<String>>>,
    compounding_reason: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReason>,
    compounding_reason_context: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReasonContext>,
    controlled_schedule: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule>,
    default_days_supply: Option<i64>,
    default_quantity: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity>,
    quantity_increment: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement>,
    default_sigs: Option<Vec<String>>,
    diagnosis: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosis>,
    medication_review: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsMedicationReview>,
    diagnosis_review: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosisReview>,
    max_refills: Option<i64>,
    notes: Option<Vec<String>>,
    pharmacy_notes: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsPharmacyNotes>,
    refills: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsRefills>,
    substitution: Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution>,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder {
    pub fn allowed_days_supply(mut self, value: Vec<i64>) -> Self {
        self.allowed_days_supply = Some(value);
        self
    }

    pub fn allowed_quantities(
        mut self,
        value: Vec<
            RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem,
        >,
    ) -> Self {
        self.allowed_quantities = Some(value);
        self
    }

    pub fn allowed_reason_categories(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedReasonCategoriesItem>,
    ) -> Self {
        self.allowed_reason_categories = Some(value);
        self
    }

    pub fn reason_category_labels(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.reason_category_labels = Some(value);
        self
    }

    pub fn compounding_reason(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn compounding_reason_context(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsCompoundingReasonContext,
    ) -> Self {
        self.compounding_reason_context = Some(value);
        self
    }

    pub fn controlled_schedule(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsControlledSchedule,
    ) -> Self {
        self.controlled_schedule = Some(value);
        self
    }

    pub fn default_days_supply(mut self, value: i64) -> Self {
        self.default_days_supply = Some(value);
        self
    }

    pub fn default_quantity(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity,
    ) -> Self {
        self.default_quantity = Some(value);
        self
    }

    pub fn quantity_increment(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement,
    ) -> Self {
        self.quantity_increment = Some(value);
        self
    }

    pub fn default_sigs(mut self, value: Vec<String>) -> Self {
        self.default_sigs = Some(value);
        self
    }

    pub fn diagnosis(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosis,
    ) -> Self {
        self.diagnosis = Some(value);
        self
    }

    pub fn medication_review(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsMedicationReview,
    ) -> Self {
        self.medication_review = Some(value);
        self
    }

    pub fn diagnosis_review(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDiagnosisReview,
    ) -> Self {
        self.diagnosis_review = Some(value);
        self
    }

    pub fn max_refills(mut self, value: i64) -> Self {
        self.max_refills = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn pharmacy_notes(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsPharmacyNotes,
    ) -> Self {
        self.pharmacy_notes = Some(value);
        self
    }

    pub fn refills(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsRefills,
    ) -> Self {
        self.refills = Some(value);
        self
    }

    pub fn substitution(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsSubstitution,
    ) -> Self {
        self.substitution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements`].
    /// This method will fail if any of the following fields are not set:
    /// - [`compounding_reason`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder::compounding_reason)
    /// - [`diagnosis`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder::diagnosis)
    /// - [`pharmacy_notes`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder::pharmacy_notes)
    /// - [`refills`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder::refills)
    /// - [`substitution`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsBuilder::substitution)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements, BuildError> {
        Ok(
            RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements {
                allowed_days_supply: self.allowed_days_supply,
                allowed_quantities: self.allowed_quantities,
                allowed_reason_categories: self.allowed_reason_categories,
                reason_category_labels: self.reason_category_labels,
                compounding_reason: self
                    .compounding_reason
                    .ok_or_else(|| BuildError::missing_field("compounding_reason"))?,
                compounding_reason_context: self.compounding_reason_context,
                controlled_schedule: self.controlled_schedule,
                default_days_supply: self.default_days_supply,
                default_quantity: self.default_quantity,
                quantity_increment: self.quantity_increment,
                default_sigs: self.default_sigs,
                diagnosis: self
                    .diagnosis
                    .ok_or_else(|| BuildError::missing_field("diagnosis"))?,
                medication_review: self.medication_review,
                diagnosis_review: self.diagnosis_review,
                max_refills: self.max_refills,
                notes: self.notes,
                pharmacy_notes: self
                    .pharmacy_notes
                    .ok_or_else(|| BuildError::missing_field("pharmacy_notes"))?,
                refills: self
                    .refills
                    .ok_or_else(|| BuildError::missing_field("refills"))?,
                substitution: self
                    .substitution
                    .ok_or_else(|| BuildError::missing_field("substitution"))?,
            },
        )
    }
}

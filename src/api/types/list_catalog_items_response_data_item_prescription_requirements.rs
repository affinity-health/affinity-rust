pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirements {
    #[serde(rename = "allowedDaysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_days_supply: Option<Vec<i64>>,
    #[serde(rename = "allowedQuantities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_quantities:
        Option<Vec<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem>>,
    #[serde(rename = "allowedReasonCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_reason_categories: Option<
        Vec<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedReasonCategoriesItem>,
    >,
    #[serde(rename = "reasonCategoryLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_category_labels: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "compoundingReason")]
    pub compounding_reason:
        ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReason,
    #[serde(rename = "compoundingReasonContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason_context:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReasonContext>,
    #[serde(rename = "controlledSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlled_schedule:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsControlledSchedule>,
    #[serde(rename = "defaultDaysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_days_supply: Option<i64>,
    #[serde(rename = "defaultQuantity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quantity:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity>,
    #[serde(rename = "quantityIncrement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_increment:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement>,
    #[serde(rename = "defaultSigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sigs: Option<Vec<String>>,
    pub diagnosis: ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosis,
    #[serde(rename = "medicationReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_review:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsMedicationReview>,
    #[serde(rename = "diagnosisReview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnosis_review:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosisReview>,
    #[serde(rename = "maxRefills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_refills: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    #[serde(rename = "pharmacyNotes")]
    pub pharmacy_notes: ListCatalogItemsResponseDataItemPrescriptionRequirementsPharmacyNotes,
    pub refills: ListCatalogItemsResponseDataItemPrescriptionRequirementsRefills,
    pub substitution: ListCatalogItemsResponseDataItemPrescriptionRequirementsSubstitution,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirements {
    pub fn builder() -> ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder {
        <ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder {
    allowed_days_supply: Option<Vec<i64>>,
    allowed_quantities:
        Option<Vec<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem>>,
    allowed_reason_categories: Option<
        Vec<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedReasonCategoriesItem>,
    >,
    reason_category_labels: Option<HashMap<String, Option<String>>>,
    compounding_reason:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReason>,
    compounding_reason_context:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReasonContext>,
    controlled_schedule:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsControlledSchedule>,
    default_days_supply: Option<i64>,
    default_quantity:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity>,
    quantity_increment:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement>,
    default_sigs: Option<Vec<String>>,
    diagnosis: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosis>,
    medication_review:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsMedicationReview>,
    diagnosis_review:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosisReview>,
    max_refills: Option<i64>,
    notes: Option<Vec<String>>,
    pharmacy_notes: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsPharmacyNotes>,
    refills: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsRefills>,
    substitution: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsSubstitution>,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder {
    pub fn allowed_days_supply(mut self, value: Vec<i64>) -> Self {
        self.allowed_days_supply = Some(value);
        self
    }

    pub fn allowed_quantities(
        mut self,
        value: Vec<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem>,
    ) -> Self {
        self.allowed_quantities = Some(value);
        self
    }

    pub fn allowed_reason_categories(
        mut self,
        value: Vec<
            ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedReasonCategoriesItem,
        >,
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
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn compounding_reason_context(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsCompoundingReasonContext,
    ) -> Self {
        self.compounding_reason_context = Some(value);
        self
    }

    pub fn controlled_schedule(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsControlledSchedule,
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
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity,
    ) -> Self {
        self.default_quantity = Some(value);
        self
    }

    pub fn quantity_increment(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement,
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
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosis,
    ) -> Self {
        self.diagnosis = Some(value);
        self
    }

    pub fn medication_review(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsMedicationReview,
    ) -> Self {
        self.medication_review = Some(value);
        self
    }

    pub fn diagnosis_review(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsDiagnosisReview,
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
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsPharmacyNotes,
    ) -> Self {
        self.pharmacy_notes = Some(value);
        self
    }

    pub fn refills(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsRefills,
    ) -> Self {
        self.refills = Some(value);
        self
    }

    pub fn substitution(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsSubstitution,
    ) -> Self {
        self.substitution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemPrescriptionRequirements`].
    /// This method will fail if any of the following fields are not set:
    /// - [`compounding_reason`](ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder::compounding_reason)
    /// - [`diagnosis`](ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder::diagnosis)
    /// - [`pharmacy_notes`](ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder::pharmacy_notes)
    /// - [`refills`](ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder::refills)
    /// - [`substitution`](ListCatalogItemsResponseDataItemPrescriptionRequirementsBuilder::substitution)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemPrescriptionRequirements, BuildError> {
        Ok(ListCatalogItemsResponseDataItemPrescriptionRequirements {
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
        })
    }
}

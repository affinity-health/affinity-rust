pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
    pub fn builder() -> AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
        <AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
    category: Option<AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason, BuildError> {
        Ok(
            AddOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

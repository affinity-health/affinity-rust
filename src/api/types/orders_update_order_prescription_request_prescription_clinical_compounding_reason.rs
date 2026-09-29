pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category:
        Option<UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
    pub fn builder() -> UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
        <UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
    category: Option<UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason, BuildError>
    {
        Ok(
            UpdateOrderPrescriptionRequestPrescriptionClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

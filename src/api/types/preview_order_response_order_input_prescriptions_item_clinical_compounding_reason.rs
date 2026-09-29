pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason {
    pub fn builder(
    ) -> PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonBuilder {
        <PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonBuilder {
    category:
        Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason, BuildError>
    {
        Ok(
            PreviewOrderResponseOrderInputPrescriptionsItemClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

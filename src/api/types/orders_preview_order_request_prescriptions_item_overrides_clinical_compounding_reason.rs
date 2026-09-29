pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonBuilder
    {
        <PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonBuilder {
    category:
        Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason, BuildError>
    {
        Ok(
            PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponsePrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub context: String,
}

impl CancelOrderResponsePrescriptionsItemClinicalCompoundingReason {
    pub fn builder() -> CancelOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
        <CancelOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
    category: Option<String>,
    context: Option<String>,
}

impl CancelOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponsePrescriptionsItemClinicalCompoundingReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context`](CancelOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder::context)
    pub fn build(
        self,
    ) -> Result<CancelOrderResponsePrescriptionsItemClinicalCompoundingReason, BuildError> {
        Ok(
            CancelOrderResponsePrescriptionsItemClinicalCompoundingReason {
                category: self.category,
                context: self
                    .context
                    .ok_or_else(|| BuildError::missing_field("context"))?,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponsePrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub context: String,
}

impl GetOrderResponsePrescriptionsItemClinicalCompoundingReason {
    pub fn builder() -> GetOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
        <GetOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
    category: Option<String>,
    context: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemClinicalCompoundingReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context`](GetOrderResponsePrescriptionsItemClinicalCompoundingReasonBuilder::context)
    pub fn build(
        self,
    ) -> Result<GetOrderResponsePrescriptionsItemClinicalCompoundingReason, BuildError> {
        Ok(GetOrderResponsePrescriptionsItemClinicalCompoundingReason {
            category: self.category,
            context: self
                .context
                .ok_or_else(|| BuildError::missing_field("context"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl CreateOrderRequestPrescriptionsItemClinicalCompoundingReason {
    pub fn builder() -> CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonBuilder {
        <CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonBuilder {
    category: Option<CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: CreateOrderRequestPrescriptionsItemClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriptionsItemClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<CreateOrderRequestPrescriptionsItemClinicalCompoundingReason, BuildError> {
        Ok(
            CreateOrderRequestPrescriptionsItemClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

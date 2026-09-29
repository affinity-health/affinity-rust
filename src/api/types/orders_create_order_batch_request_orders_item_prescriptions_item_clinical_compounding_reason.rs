pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason {
    pub fn builder(
    ) -> CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonBuilder {
        <CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonBuilder {
    category:
        Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonCategory>,
    context: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReasonCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason`].
    pub fn build(
        self,
    ) -> Result<
        CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason,
        BuildError,
    > {
        Ok(
            CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalCompoundingReason {
                category: self.category,
                context: self.context,
            },
        )
    }
}

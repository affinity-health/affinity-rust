pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub context: String,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason {
    pub fn builder() -> ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReasonBuilder
    {
        <ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReasonBuilder {
    category: Option<String>,
    context: Option<String>,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReasonBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context`](ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReasonBuilder::context)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason, BuildError>
    {
        Ok(
            ListOrdersResponseDataItemPrescriptionsItemClinicalCompoundingReason {
                category: self.category,
                context: self
                    .context
                    .ok_or_else(|| BuildError::missing_field("context"))?,
            },
        )
    }
}

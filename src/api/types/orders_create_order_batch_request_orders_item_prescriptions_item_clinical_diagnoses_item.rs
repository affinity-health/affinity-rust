pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem {
    pub fn builder(
    ) -> CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder {
        <CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder::code)
    /// - [`display`](CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem, BuildError>
    {
        Ok(
            CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalDiagnosesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
            },
        )
    }
}

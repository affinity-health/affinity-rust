pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem {
    pub fn builder(
    ) -> CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder {
        <CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder {
    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`unit`](CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder::unit)
    /// - [`value`](CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem,
        BuildError,
    > {
        Ok(
            CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItem {
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderRequestPrescriptionsItemClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue,
}

impl CreateOrderRequestPrescriptionsItemClinicalObservationsItem {
    pub fn builder() -> CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder {
        <CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue>,
}

impl CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder {
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
        value: CreateOrderRequestPrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`unit`](CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder::unit)
    /// - [`value`](CreateOrderRequestPrescriptionsItemClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<CreateOrderRequestPrescriptionsItemClinicalObservationsItem, BuildError> {
        Ok(
            CreateOrderRequestPrescriptionsItemClinicalObservationsItem {
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

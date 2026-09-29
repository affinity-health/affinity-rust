pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem {
    pub fn builder(
    ) -> PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder {
        <PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder {
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
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`unit`](PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder::unit)
    /// - [`value`](PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem, BuildError>
    {
        Ok(
            PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItem {
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

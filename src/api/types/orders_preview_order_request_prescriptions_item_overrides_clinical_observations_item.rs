pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder
    {
        <PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder {
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
        value: PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder::display)
    /// - [`unit`](PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder::unit)
    /// - [`value`](PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem, BuildError>
    {
        Ok(
            PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
    pub fn builder() -> UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
        <UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue>,
}

impl UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
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
        value: UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::display)
    /// - [`unit`](UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::unit)
    /// - [`value`](UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem, BuildError>
    {
        Ok(
            UpdateOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(default)]
    pub unit: String,
    pub value: AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
    pub fn builder() -> AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
        <AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
    display: Option<String>,
    unit: Option<String>,
    value: Option<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue>,
}

impl AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder {
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
        value: AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::display)
    /// - [`unit`](AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::unit)
    /// - [`value`](AddOrderPrescriptionRequestPrescriptionClinicalObservationsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem, BuildError> {
        Ok(
            AddOrderPrescriptionRequestPrescriptionClinicalObservationsItem {
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

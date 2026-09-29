pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelOrderResponsePrescriptionsItemClinicalObservationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub display: String,
    pub value: CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue,
    #[serde(default)]
    pub unit: String,
}

impl CancelOrderResponsePrescriptionsItemClinicalObservationsItem {
    pub fn builder() -> CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
        <CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
    code: Option<String>,
    display: Option<String>,
    value: Option<CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue>,
    unit: Option<String>,
}

impl CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: CancelOrderResponsePrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponsePrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`value`](CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::value)
    /// - [`unit`](CancelOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::unit)
    pub fn build(
        self,
    ) -> Result<CancelOrderResponsePrescriptionsItemClinicalObservationsItem, BuildError> {
        Ok(
            CancelOrderResponsePrescriptionsItemClinicalObservationsItem {
                code: self.code,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}

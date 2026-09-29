pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetOrderResponsePrescriptionsItemClinicalObservationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub display: String,
    pub value: GetOrderResponsePrescriptionsItemClinicalObservationsItemValue,
    #[serde(default)]
    pub unit: String,
}

impl GetOrderResponsePrescriptionsItemClinicalObservationsItem {
    pub fn builder() -> GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
        <GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
    code: Option<String>,
    display: Option<String>,
    value: Option<GetOrderResponsePrescriptionsItemClinicalObservationsItemValue>,
    unit: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder {
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
        value: GetOrderResponsePrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`value`](GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::value)
    /// - [`unit`](GetOrderResponsePrescriptionsItemClinicalObservationsItemBuilder::unit)
    pub fn build(
        self,
    ) -> Result<GetOrderResponsePrescriptionsItemClinicalObservationsItem, BuildError> {
        Ok(GetOrderResponsePrescriptionsItemClinicalObservationsItem {
            code: self.code,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}

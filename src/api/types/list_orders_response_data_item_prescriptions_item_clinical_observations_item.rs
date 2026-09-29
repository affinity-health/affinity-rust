pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub display: String,
    pub value: ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue,
    #[serde(default)]
    pub unit: String,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem {
    pub fn builder() -> ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder {
        <ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder {
    code: Option<String>,
    display: Option<String>,
    value: Option<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue>,
    unit: Option<String>,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder {
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
        value: ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder::display)
    /// - [`value`](ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder::value)
    /// - [`unit`](ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemBuilder::unit)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem, BuildError>
    {
        Ok(
            ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem {
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i64>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub unit: String,
    pub value: ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemValue,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder {
        <ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder {
    days_supply: Option<i64>,
    label: Option<String>,
    unit: Option<String>,
    value:
        Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemValue>,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder {
    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder::label)
    /// - [`unit`](ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder::unit)
    /// - [`value`](ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem,
        BuildError,
    > {
        Ok(
            ListCatalogItemsResponseDataItemPrescriptionRequirementsAllowedQuantitiesItem {
                days_supply: self.days_supply,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}

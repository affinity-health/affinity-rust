pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin>,
    #[serde(default)]
    pub unit: String,
    pub value: ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder {
        <ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder {
    max: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax>,
    min: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin>,
    unit: Option<String>,
    value: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue>,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder {
    pub fn max(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMax,
    ) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementMin,
    ) -> Self {
        self.min = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder::unit)
    /// - [`value`](ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrementBuilder::value)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement, BuildError>
    {
        Ok(
            ListCatalogItemsResponseDataItemPrescriptionRequirementsQuantityIncrement {
                max: self.max,
                min: self.min,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity {
    #[serde(default)]
    pub unit: String,
    pub value: ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder {
        <ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder {
    unit: Option<String>,
    value: Option<ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue>,
}

impl ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder::unit)
    /// - [`value`](ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantityBuilder::value)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity, BuildError>
    {
        Ok(
            ListCatalogItemsResponseDataItemPrescriptionRequirementsDefaultQuantity {
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}

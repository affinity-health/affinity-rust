pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer {
    #[serde(default)]
    pub value: String,
    pub unit:
        ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer {
    pub fn builder() -> ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder{
        <ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder
{
    value: Option<String>,
    unit: Option<
        ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit,
    >,
}

impl
    ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder
{
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(
        mut self,
        value: ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerUnit,
    ) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainerBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer,
        BuildError,
    > {
        Ok(ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}

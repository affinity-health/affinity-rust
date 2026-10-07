pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer {
    #[serde(default)]
    pub value: String,
    pub unit: RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerUnit,
}

impl
    RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer
{
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder{
        <RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder {
    value: Option<String>,
    unit: Option<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerUnit>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerUnit) -> Self {
        self.unit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainerBuilder::unit)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem {
    #[serde(default)]
    pub container: String,
    #[serde(rename = "containerCount")]
    #[serde(default)]
    pub container_count: i64,
    #[serde(rename = "contentsPerContainer")]
    pub contents_per_container: RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder {
        <RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder {
    container: Option<String>,
    container_count: Option<i64>,
    contents_per_container: Option<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder {
    pub fn container(mut self, value: impl Into<String>) -> Self {
        self.container = Some(value.into());
        self
    }

    pub fn container_count(mut self, value: i64) -> Self {
        self.container_count = Some(value);
        self
    }

    pub fn contents_per_container(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemContentsPerContainer,
    ) -> Self {
        self.contents_per_container = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`container`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder::container)
    /// - [`container_count`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder::container_count)
    /// - [`contents_per_container`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItemBuilder::contents_per_container)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem,
        BuildError,
    > {
        Ok(
            RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem {
                container: self
                    .container
                    .ok_or_else(|| BuildError::missing_field("container"))?,
                container_count: self
                    .container_count
                    .ok_or_else(|| BuildError::missing_field("container_count"))?,
                contents_per_container: self
                    .contents_per_container
                    .ok_or_else(|| BuildError::missing_field("contents_per_container"))?,
            },
        )
    }
}

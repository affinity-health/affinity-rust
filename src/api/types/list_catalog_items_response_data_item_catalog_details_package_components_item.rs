pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem {
    #[serde(default)]
    pub container: String,
    #[serde(rename = "containerCount")]
    #[serde(default)]
    pub container_count: i64,
    #[serde(rename = "contentsPerContainer")]
    pub contents_per_container:
        ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem {
    pub fn builder() -> ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder {
        <ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder {
    container: Option<String>,
    container_count: Option<i64>,
    contents_per_container: Option<
        ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer,
    >,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder {
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
        value: ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemContentsPerContainer,
    ) -> Self {
        self.contents_per_container = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`container`](ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder::container)
    /// - [`container_count`](ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder::container_count)
    /// - [`contents_per_container`](ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItemBuilder::contents_per_container)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem, BuildError>
    {
        Ok(
            ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem {
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

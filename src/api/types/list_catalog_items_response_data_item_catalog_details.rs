pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCatalogItemsResponseDataItemCatalogDetails {
    /// Confirmed physical containers and contents. Empty or absent means container count cannot be inferred from dispense quantity.
    #[serde(rename = "packageComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_components:
        Option<Vec<ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem>>,
    #[serde(default)]
    pub attributes: HashMap<String, ListCatalogItemsResponseDataItemCatalogDetailsAttributesValue>,
    #[serde(default)]
    pub directions: Vec<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem>,
}

impl ListCatalogItemsResponseDataItemCatalogDetails {
    pub fn builder() -> ListCatalogItemsResponseDataItemCatalogDetailsBuilder {
        <ListCatalogItemsResponseDataItemCatalogDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCatalogDetailsBuilder {
    package_components:
        Option<Vec<ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem>>,
    attributes:
        Option<HashMap<String, ListCatalogItemsResponseDataItemCatalogDetailsAttributesValue>>,
    directions: Option<Vec<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem>>,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsBuilder {
    pub fn package_components(
        mut self,
        value: Vec<ListCatalogItemsResponseDataItemCatalogDetailsPackageComponentsItem>,
    ) -> Self {
        self.package_components = Some(value);
        self
    }

    pub fn attributes(
        mut self,
        value: HashMap<String, ListCatalogItemsResponseDataItemCatalogDetailsAttributesValue>,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn directions(
        mut self,
        value: Vec<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem>,
    ) -> Self {
        self.directions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemCatalogDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](ListCatalogItemsResponseDataItemCatalogDetailsBuilder::attributes)
    /// - [`directions`](ListCatalogItemsResponseDataItemCatalogDetailsBuilder::directions)
    pub fn build(self) -> Result<ListCatalogItemsResponseDataItemCatalogDetails, BuildError> {
        Ok(ListCatalogItemsResponseDataItemCatalogDetails {
            package_components: self.package_components,
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}

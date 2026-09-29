pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCatalogItemsResponseDataItemCatalogDetails {
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
    attributes:
        Option<HashMap<String, ListCatalogItemsResponseDataItemCatalogDetailsAttributesValue>>,
    directions: Option<Vec<ListCatalogItemsResponseDataItemCatalogDetailsDirectionsItem>>,
}

impl ListCatalogItemsResponseDataItemCatalogDetailsBuilder {
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
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}

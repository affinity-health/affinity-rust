pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCatalogItemsResponseDataItemComposition {
    pub status: ListCatalogItemsResponseDataItemCompositionStatus,
    #[serde(default)]
    pub ingredients: Vec<ListCatalogItemsResponseDataItemCompositionIngredientsItem>,
}

impl ListCatalogItemsResponseDataItemComposition {
    pub fn builder() -> ListCatalogItemsResponseDataItemCompositionBuilder {
        <ListCatalogItemsResponseDataItemCompositionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemCompositionBuilder {
    status: Option<ListCatalogItemsResponseDataItemCompositionStatus>,
    ingredients: Option<Vec<ListCatalogItemsResponseDataItemCompositionIngredientsItem>>,
}

impl ListCatalogItemsResponseDataItemCompositionBuilder {
    pub fn status(mut self, value: ListCatalogItemsResponseDataItemCompositionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn ingredients(
        mut self,
        value: Vec<ListCatalogItemsResponseDataItemCompositionIngredientsItem>,
    ) -> Self {
        self.ingredients = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemComposition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ListCatalogItemsResponseDataItemCompositionBuilder::status)
    /// - [`ingredients`](ListCatalogItemsResponseDataItemCompositionBuilder::ingredients)
    pub fn build(self) -> Result<ListCatalogItemsResponseDataItemComposition, BuildError> {
        Ok(ListCatalogItemsResponseDataItemComposition {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            ingredients: self
                .ingredients
                .ok_or_else(|| BuildError::missing_field("ingredients"))?,
        })
    }
}

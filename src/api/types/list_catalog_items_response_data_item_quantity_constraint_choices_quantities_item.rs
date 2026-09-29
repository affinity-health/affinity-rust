pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItem {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItem {
    pub fn builder(
    ) -> ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder {
        <ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItemBuilder::unit)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItem, BuildError>
    {
        Ok(
            ListCatalogItemsResponseDataItemQuantityConstraintChoicesQuantitiesItem {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}

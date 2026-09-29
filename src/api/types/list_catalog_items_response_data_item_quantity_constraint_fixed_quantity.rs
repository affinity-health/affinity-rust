pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantity {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantity {
    pub fn builder() -> ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder {
        <ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder::value)
    /// - [`unit`](ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantityBuilder::unit)
    pub fn build(
        self,
    ) -> Result<ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantity, BuildError> {
        Ok(
            ListCatalogItemsResponseDataItemQuantityConstraintFixedQuantity {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}

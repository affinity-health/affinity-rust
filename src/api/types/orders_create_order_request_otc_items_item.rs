pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestOtcItemsItem {
    #[serde(rename = "catalogItemId")]
    #[serde(default)]
    pub catalog_item_id: String,
    #[serde(default)]
    pub quantity: i64,
}

impl CreateOrderRequestOtcItemsItem {
    pub fn builder() -> CreateOrderRequestOtcItemsItemBuilder {
        <CreateOrderRequestOtcItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestOtcItemsItemBuilder {
    catalog_item_id: Option<String>,
    quantity: Option<i64>,
}

impl CreateOrderRequestOtcItemsItemBuilder {
    pub fn catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_item_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestOtcItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_item_id`](CreateOrderRequestOtcItemsItemBuilder::catalog_item_id)
    /// - [`quantity`](CreateOrderRequestOtcItemsItemBuilder::quantity)
    pub fn build(self) -> Result<CreateOrderRequestOtcItemsItem, BuildError> {
        Ok(CreateOrderRequestOtcItemsItem {
            catalog_item_id: self
                .catalog_item_id
                .ok_or_else(|| BuildError::missing_field("catalog_item_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}

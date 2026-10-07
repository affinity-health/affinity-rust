pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum ListCatalogItemsResponseDataItemPricingBasis {
    #[serde(rename = "item")]
    #[non_exhaustive]
    Item {
        quantity: ListCatalogItemsResponseDataItemPricingBasisItemQuantity,
        #[serde(default)]
        unit: String,
        #[serde(rename = "quantityPrices")]
        #[serde(skip_serializing_if = "Option::is_none")]
        quantity_prices:
            Option<Vec<ListCatalogItemsResponseDataItemPricingBasisItemQuantityPricesItem>>,
    },

    #[serde(rename = "package")]
    #[non_exhaustive]
    Package {
        #[serde(default)]
        quantity: String,
        #[serde(default)]
        unit: String,
    },

    #[serde(rename = "unit")]
    #[non_exhaustive]
    Unit {
        quantity: ListCatalogItemsResponseDataItemPricingBasisUnitQuantity,
        #[serde(default)]
        unit: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ListCatalogItemsResponseDataItemPricingBasis {
    pub fn item(
        quantity: ListCatalogItemsResponseDataItemPricingBasisItemQuantity,
        unit: String,
    ) -> Self {
        Self::Item {
            quantity,
            unit,
            quantity_prices: None,
        }
    }

    pub fn package(quantity: String, unit: String) -> Self {
        Self::Package { quantity, unit }
    }

    pub fn unit(
        quantity: ListCatalogItemsResponseDataItemPricingBasisUnitQuantity,
        unit: String,
    ) -> Self {
        Self::Unit { quantity, unit }
    }

    pub fn item_with_quantity_prices(
        quantity: ListCatalogItemsResponseDataItemPricingBasisItemQuantity,
        unit: String,
        quantity_prices: Vec<ListCatalogItemsResponseDataItemPricingBasisItemQuantityPricesItem>,
    ) -> Self {
        Self::Item {
            quantity,
            unit,
            quantity_prices: Some(quantity_prices),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

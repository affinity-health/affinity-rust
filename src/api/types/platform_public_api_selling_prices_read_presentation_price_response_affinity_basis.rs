pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasis {
    #[serde(rename = "item")]
        #[non_exhaustive]
        Item {
            quantity: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantity,
            #[serde(default)]
            unit: String,
            #[serde(rename = "quantityPrices")]
            #[serde(skip_serializing_if = "Option::is_none")]
            quantity_prices: Option<Vec<PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem>>,
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
            quantity: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisUnitQuantity,
            #[serde(default)]
            unit: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasis {
    pub fn item(
        quantity: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantity,
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
        quantity: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisUnitQuantity,
        unit: String,
    ) -> Self {
        Self::Unit { quantity, unit }
    }

    pub fn item_with_quantity_prices(
        quantity: PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantity,
        unit: String,
        quantity_prices: Vec<PlatformPublicApiSellingPricesReadPresentationPriceResponseAffinityBasisItemQuantityPricesItem>,
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

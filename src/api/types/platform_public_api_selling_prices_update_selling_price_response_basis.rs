pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasis {
    #[serde(rename = "item")]
    #[non_exhaustive]
    Item {
        quantity: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisItemQuantity,
        #[serde(default)]
        unit: String,
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
        quantity: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisUnitQuantity,
        #[serde(default)]
        unit: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasis {
    pub fn item(
        quantity: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisItemQuantity,
        unit: String,
    ) -> Self {
        Self::Item { quantity, unit }
    }

    pub fn package(quantity: String, unit: String) -> Self {
        Self::Package { quantity, unit }
    }

    pub fn unit(
        quantity: PlatformPublicApiSellingPricesUpdateSellingPriceResponseBasisUnitQuantity,
        unit: String,
    ) -> Self {
        Self::Unit { quantity, unit }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

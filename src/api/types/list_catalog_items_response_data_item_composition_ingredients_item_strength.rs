pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum ListCatalogItemsResponseDataItemCompositionIngredientsItemStrength {
    #[serde(rename = "amount")]
    #[non_exhaustive]
    Amount {
        #[serde(default)]
        amount: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount,
    },

    #[serde(rename = "ratio")]
    #[non_exhaustive]
    Ratio {
        #[serde(default)]
        numerator: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator,
        #[serde(default)]
        denominator:
            ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator,
    },

    #[serde(rename = "unresolved")]
    #[non_exhaustive]
    Unresolved {
        #[serde(default)]
        reason: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ListCatalogItemsResponseDataItemCompositionIngredientsItemStrength {
    pub fn amount(
        amount: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthAmountAmount,
    ) -> Self {
        Self::Amount { amount }
    }

    pub fn ratio(
        numerator: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioNumerator,
        denominator: ListCatalogItemsResponseDataItemCompositionIngredientsItemStrengthRatioDenominator,
    ) -> Self {
        Self::Ratio {
            numerator,
            denominator,
        }
    }

    pub fn unresolved(reason: String) -> Self {
        Self::Unresolved { reason }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

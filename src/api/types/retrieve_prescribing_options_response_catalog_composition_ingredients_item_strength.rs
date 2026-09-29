pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrength {
    #[serde(rename = "amount")]
        #[non_exhaustive]
        Amount {
            #[serde(default)]
            amount: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount,
        },

        #[serde(rename = "ratio")]
        #[non_exhaustive]
        Ratio {
            #[serde(default)]
            numerator: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator,
            #[serde(default)]
            denominator: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator,
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

impl RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrength {
    pub fn amount(
        amount: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthAmountAmount,
    ) -> Self {
        Self::Amount { amount }
    }

    pub fn ratio(
        numerator: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioNumerator,
        denominator: RetrievePrescribingOptionsResponseCatalogCompositionIngredientsItemStrengthRatioDenominator,
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

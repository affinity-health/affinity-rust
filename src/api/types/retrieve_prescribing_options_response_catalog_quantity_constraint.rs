pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum RetrievePrescribingOptionsResponseCatalogQuantityConstraint {
    #[serde(rename = "fixed")]
    #[non_exhaustive]
    Fixed {
        #[serde(default)]
        quantity: RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity,
    },

    #[serde(rename = "choices")]
    #[non_exhaustive]
    Choices {
        #[serde(default)]
        quantities:
            Vec<RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem>,
    },

    #[serde(rename = "range")]
    #[non_exhaustive]
    Range {
        #[serde(default)]
        unit: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        minimum: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        maximum: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        increment: Option<String>,
    },

    #[serde(rename = "unresolved")]
    #[non_exhaustive]
    Unresolved {
        #[serde(rename = "sourceText")]
        #[serde(default)]
        source_text: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl RetrievePrescribingOptionsResponseCatalogQuantityConstraint {
    pub fn fixed(
        quantity: RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity,
    ) -> Self {
        Self::Fixed { quantity }
    }

    pub fn choices(
        quantities: Vec<
            RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem,
        >,
    ) -> Self {
        Self::Choices { quantities }
    }

    pub fn range(unit: String) -> Self {
        Self::Range {
            unit,
            minimum: None,
            maximum: None,
            increment: None,
        }
    }

    pub fn unresolved(source_text: String) -> Self {
        Self::Unresolved { source_text }
    }

    pub fn range_with_minimum(
        unit: String,
        minimum: String,
        maximum: Option<String>,
        increment: Option<String>,
    ) -> Self {
        Self::Range {
            unit,
            minimum: Some(minimum),
            maximum,
            increment,
        }
    }

    pub fn range_with_maximum(
        unit: String,
        minimum: Option<String>,
        maximum: String,
        increment: Option<String>,
    ) -> Self {
        Self::Range {
            unit,
            minimum,
            maximum: Some(maximum),
            increment,
        }
    }

    pub fn range_with_increment(
        unit: String,
        minimum: Option<String>,
        maximum: Option<String>,
        increment: String,
    ) -> Self {
        Self::Range {
            unit,
            minimum,
            maximum,
            increment: Some(increment),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

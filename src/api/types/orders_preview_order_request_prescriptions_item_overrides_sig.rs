pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format")]
#[non_exhaustive]
pub enum PreviewOrderRequestPrescriptionsItemOverridesSig {
    #[serde(rename = "structured")]
    #[non_exhaustive]
    Structured {
        #[serde(default)]
        fields: PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields,
    },

    #[serde(rename = "free_text")]
    #[non_exhaustive]
    FreeText {
        #[serde(default)]
        text: String,
    },

    #[serde(rename = "template")]
    #[non_exhaustive]
    Template {
        #[serde(rename = "templateId")]
        #[serde(default)]
        template_id: String,
        #[serde(rename = "templateRevision")]
        #[serde(default)]
        template_revision: String,
        #[serde(default)]
        values: HashMap<String, String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PreviewOrderRequestPrescriptionsItemOverridesSig {
    pub fn structured(
        fields: PreviewOrderRequestPrescriptionsItemOverridesSigStructuredFields,
    ) -> Self {
        Self::Structured { fields }
    }

    pub fn free_text(text: String) -> Self {
        Self::FreeText { text }
    }

    pub fn template(
        template_id: String,
        template_revision: String,
        values: HashMap<String, String>,
    ) -> Self {
        Self::Template {
            template_id,
            template_revision,
            values,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}

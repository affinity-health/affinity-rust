pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem {
    pub category: RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemCategory,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "contextRequired")]
    #[serde(default)]
    pub context_required: bool,
    #[serde(rename = "contextPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_prompt: Option<String>,
}

impl RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder {
        <RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder {
    category: Option<RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemCategory>,
    label: Option<String>,
    context_required: Option<bool>,
    context_prompt: Option<String>,
}

impl RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder {
    pub fn category(
        mut self,
        value: RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn context_required(mut self, value: bool) -> Self {
        self.context_required = Some(value);
        self
    }

    pub fn context_prompt(mut self, value: impl Into<String>) -> Self {
        self.context_prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder::category)
    /// - [`label`](RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder::label)
    /// - [`context_required`](RetrievePrescribingOptionsResponseCompoundingReasonChoicesItemBuilder::context_required)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem, BuildError> {
        Ok(
            RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem {
                category: self
                    .category
                    .ok_or_else(|| BuildError::missing_field("category"))?,
                label: self
                    .label
                    .ok_or_else(|| BuildError::missing_field("label"))?,
                context_required: self
                    .context_required
                    .ok_or_else(|| BuildError::missing_field("context_required"))?,
                context_prompt: self.context_prompt,
            },
        )
    }
}

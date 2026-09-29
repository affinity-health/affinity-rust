pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCompoundingReason {
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "categoryRequired")]
    #[serde(default)]
    pub category_required: bool,
    pub context: RetrievePrescribingOptionsResponseCompoundingReasonContext,
    #[serde(rename = "contextPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_prompt: Option<String>,
    #[serde(default)]
    pub choices: Vec<RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem>,
}

impl RetrievePrescribingOptionsResponseCompoundingReason {
    pub fn builder() -> RetrievePrescribingOptionsResponseCompoundingReasonBuilder {
        <RetrievePrescribingOptionsResponseCompoundingReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCompoundingReasonBuilder {
    required: Option<bool>,
    category_required: Option<bool>,
    context: Option<RetrievePrescribingOptionsResponseCompoundingReasonContext>,
    context_prompt: Option<String>,
    choices: Option<Vec<RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem>>,
}

impl RetrievePrescribingOptionsResponseCompoundingReasonBuilder {
    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn category_required(mut self, value: bool) -> Self {
        self.category_required = Some(value);
        self
    }

    pub fn context(
        mut self,
        value: RetrievePrescribingOptionsResponseCompoundingReasonContext,
    ) -> Self {
        self.context = Some(value);
        self
    }

    pub fn context_prompt(mut self, value: impl Into<String>) -> Self {
        self.context_prompt = Some(value.into());
        self
    }

    pub fn choices(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCompoundingReasonChoicesItem>,
    ) -> Self {
        self.choices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCompoundingReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`required`](RetrievePrescribingOptionsResponseCompoundingReasonBuilder::required)
    /// - [`category_required`](RetrievePrescribingOptionsResponseCompoundingReasonBuilder::category_required)
    /// - [`context`](RetrievePrescribingOptionsResponseCompoundingReasonBuilder::context)
    /// - [`choices`](RetrievePrescribingOptionsResponseCompoundingReasonBuilder::choices)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCompoundingReason, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCompoundingReason {
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            category_required: self
                .category_required
                .ok_or_else(|| BuildError::missing_field("category_required"))?,
            context: self
                .context
                .ok_or_else(|| BuildError::missing_field("context"))?,
            context_prompt: self.context_prompt,
            choices: self
                .choices
                .ok_or_else(|| BuildError::missing_field("choices"))?,
        })
    }
}

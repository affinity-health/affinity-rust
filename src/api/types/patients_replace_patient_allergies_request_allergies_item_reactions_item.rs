pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesRequestAllergiesItemReactionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<ReplacePatientAllergiesRequestAllergiesItemReactionsItemCodeSystem>,
    #[serde(default)]
    pub display: String,
}

impl ReplacePatientAllergiesRequestAllergiesItemReactionsItem {
    pub fn builder() -> ReplacePatientAllergiesRequestAllergiesItemReactionsItemBuilder {
        <ReplacePatientAllergiesRequestAllergiesItemReactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesRequestAllergiesItemReactionsItemBuilder {
    code: Option<String>,
    code_system: Option<ReplacePatientAllergiesRequestAllergiesItemReactionsItemCodeSystem>,
    display: Option<String>,
}

impl ReplacePatientAllergiesRequestAllergiesItemReactionsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(
        mut self,
        value: ReplacePatientAllergiesRequestAllergiesItemReactionsItemCodeSystem,
    ) -> Self {
        self.code_system = Some(value);
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesRequestAllergiesItemReactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](ReplacePatientAllergiesRequestAllergiesItemReactionsItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<ReplacePatientAllergiesRequestAllergiesItemReactionsItem, BuildError> {
        Ok(ReplacePatientAllergiesRequestAllergiesItemReactionsItem {
            code: self.code,
            code_system: self.code_system,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
        })
    }
}

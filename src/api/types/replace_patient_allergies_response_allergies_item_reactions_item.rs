pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesResponseAllergiesItemReactionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<ReplacePatientAllergiesResponseAllergiesItemReactionsItemCodeSystem>,
    #[serde(default)]
    pub display: String,
}

impl ReplacePatientAllergiesResponseAllergiesItemReactionsItem {
    pub fn builder() -> ReplacePatientAllergiesResponseAllergiesItemReactionsItemBuilder {
        <ReplacePatientAllergiesResponseAllergiesItemReactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesResponseAllergiesItemReactionsItemBuilder {
    code: Option<String>,
    code_system: Option<ReplacePatientAllergiesResponseAllergiesItemReactionsItemCodeSystem>,
    display: Option<String>,
}

impl ReplacePatientAllergiesResponseAllergiesItemReactionsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(
        mut self,
        value: ReplacePatientAllergiesResponseAllergiesItemReactionsItemCodeSystem,
    ) -> Self {
        self.code_system = Some(value);
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesResponseAllergiesItemReactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](ReplacePatientAllergiesResponseAllergiesItemReactionsItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<ReplacePatientAllergiesResponseAllergiesItemReactionsItem, BuildError> {
        Ok(ReplacePatientAllergiesResponseAllergiesItemReactionsItem {
            code: self.code,
            code_system: self.code_system,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
        })
    }
}

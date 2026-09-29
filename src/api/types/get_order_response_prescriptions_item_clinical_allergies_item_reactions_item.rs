pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<String>,
    #[serde(default)]
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem {
    pub fn builder() -> GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItemBuilder {
        <GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItemBuilder {
    code: Option<String>,
    code_system: Option<String>,
    display: Option<String>,
    source: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(mut self, value: impl Into<String>) -> Self {
        self.code_system = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem, BuildError>
    {
        Ok(
            GetOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem {
                code: self.code,
                code_system: self.code_system,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
                source: self.source,
            },
        )
    }
}

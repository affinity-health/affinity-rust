pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponsePrescriptionsItemClinicalAllergiesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<String>,
    #[serde(default)]
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "verificationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions:
        Option<Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem>>,
}

impl CancelOrderResponsePrescriptionsItemClinicalAllergiesItem {
    pub fn builder() -> CancelOrderResponsePrescriptionsItemClinicalAllergiesItemBuilder {
        <CancelOrderResponsePrescriptionsItemClinicalAllergiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponsePrescriptionsItemClinicalAllergiesItemBuilder {
    code: Option<String>,
    code_system: Option<String>,
    display: Option<String>,
    source: Option<String>,
    category: Option<String>,
    severity: Option<String>,
    r#type: Option<String>,
    verification_status: Option<String>,
    reactions: Option<Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem>>,
}

impl CancelOrderResponsePrescriptionsItemClinicalAllergiesItemBuilder {
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

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn verification_status(mut self, value: impl Into<String>) -> Self {
        self.verification_status = Some(value.into());
        self
    }

    pub fn reactions(
        mut self,
        value: Vec<CancelOrderResponsePrescriptionsItemClinicalAllergiesItemReactionsItem>,
    ) -> Self {
        self.reactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponsePrescriptionsItemClinicalAllergiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](CancelOrderResponsePrescriptionsItemClinicalAllergiesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<CancelOrderResponsePrescriptionsItemClinicalAllergiesItem, BuildError> {
        Ok(CancelOrderResponsePrescriptionsItemClinicalAllergiesItem {
            code: self.code,
            code_system: self.code_system,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
            source: self.source,
            category: self.category,
            severity: self.severity,
            r#type: self.r#type,
            verification_status: self.verification_status,
            reactions: self.reactions,
        })
    }
}

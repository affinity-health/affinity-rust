pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesRequestAllergiesItem {
    pub category: ReplacePatientAllergiesRequestAllergiesItemCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<ReplacePatientAllergiesRequestAllergiesItemCodeSystem>,
    #[serde(default)]
    pub reactions: Vec<ReplacePatientAllergiesRequestAllergiesItemReactionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ReplacePatientAllergiesRequestAllergiesItemSeverity>,
    pub source: ReplacePatientAllergiesRequestAllergiesItemSource,
    #[serde(default)]
    pub substance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReplacePatientAllergiesRequestAllergiesItemType>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: ReplacePatientAllergiesRequestAllergiesItemVerificationStatus,
}

impl ReplacePatientAllergiesRequestAllergiesItem {
    pub fn builder() -> ReplacePatientAllergiesRequestAllergiesItemBuilder {
        <ReplacePatientAllergiesRequestAllergiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesRequestAllergiesItemBuilder {
    category: Option<ReplacePatientAllergiesRequestAllergiesItemCategory>,
    code: Option<String>,
    code_system: Option<ReplacePatientAllergiesRequestAllergiesItemCodeSystem>,
    reactions: Option<Vec<ReplacePatientAllergiesRequestAllergiesItemReactionsItem>>,
    severity: Option<ReplacePatientAllergiesRequestAllergiesItemSeverity>,
    source: Option<ReplacePatientAllergiesRequestAllergiesItemSource>,
    substance: Option<String>,
    r#type: Option<ReplacePatientAllergiesRequestAllergiesItemType>,
    verification_status: Option<ReplacePatientAllergiesRequestAllergiesItemVerificationStatus>,
}

impl ReplacePatientAllergiesRequestAllergiesItemBuilder {
    pub fn category(mut self, value: ReplacePatientAllergiesRequestAllergiesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(
        mut self,
        value: ReplacePatientAllergiesRequestAllergiesItemCodeSystem,
    ) -> Self {
        self.code_system = Some(value);
        self
    }

    pub fn reactions(
        mut self,
        value: Vec<ReplacePatientAllergiesRequestAllergiesItemReactionsItem>,
    ) -> Self {
        self.reactions = Some(value);
        self
    }

    pub fn severity(mut self, value: ReplacePatientAllergiesRequestAllergiesItemSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn source(mut self, value: ReplacePatientAllergiesRequestAllergiesItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn substance(mut self, value: impl Into<String>) -> Self {
        self.substance = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ReplacePatientAllergiesRequestAllergiesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn verification_status(
        mut self,
        value: ReplacePatientAllergiesRequestAllergiesItemVerificationStatus,
    ) -> Self {
        self.verification_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesRequestAllergiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](ReplacePatientAllergiesRequestAllergiesItemBuilder::category)
    /// - [`reactions`](ReplacePatientAllergiesRequestAllergiesItemBuilder::reactions)
    /// - [`source`](ReplacePatientAllergiesRequestAllergiesItemBuilder::source)
    /// - [`substance`](ReplacePatientAllergiesRequestAllergiesItemBuilder::substance)
    /// - [`verification_status`](ReplacePatientAllergiesRequestAllergiesItemBuilder::verification_status)
    pub fn build(self) -> Result<ReplacePatientAllergiesRequestAllergiesItem, BuildError> {
        Ok(ReplacePatientAllergiesRequestAllergiesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            code: self.code,
            code_system: self.code_system,
            reactions: self
                .reactions
                .ok_or_else(|| BuildError::missing_field("reactions"))?,
            severity: self.severity,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            substance: self
                .substance
                .ok_or_else(|| BuildError::missing_field("substance"))?,
            r#type: self.r#type,
            verification_status: self
                .verification_status
                .ok_or_else(|| BuildError::missing_field("verification_status"))?,
        })
    }
}

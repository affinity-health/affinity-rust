pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesResponseAllergiesItem {
    pub category: ReplacePatientAllergiesResponseAllergiesItemCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<ReplacePatientAllergiesResponseAllergiesItemCodeSystem>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reactions: Vec<ReplacePatientAllergiesResponseAllergiesItemReactionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<ReplacePatientAllergiesResponseAllergiesItemSeverity>,
    pub source: ReplacePatientAllergiesResponseAllergiesItemSource,
    #[serde(default)]
    pub substance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReplacePatientAllergiesResponseAllergiesItemType>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: ReplacePatientAllergiesResponseAllergiesItemVerificationStatus,
}

impl ReplacePatientAllergiesResponseAllergiesItem {
    pub fn builder() -> ReplacePatientAllergiesResponseAllergiesItemBuilder {
        <ReplacePatientAllergiesResponseAllergiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesResponseAllergiesItemBuilder {
    category: Option<ReplacePatientAllergiesResponseAllergiesItemCategory>,
    code: Option<String>,
    code_system: Option<ReplacePatientAllergiesResponseAllergiesItemCodeSystem>,
    id: Option<String>,
    reactions: Option<Vec<ReplacePatientAllergiesResponseAllergiesItemReactionsItem>>,
    severity: Option<ReplacePatientAllergiesResponseAllergiesItemSeverity>,
    source: Option<ReplacePatientAllergiesResponseAllergiesItemSource>,
    substance: Option<String>,
    r#type: Option<ReplacePatientAllergiesResponseAllergiesItemType>,
    verification_status: Option<ReplacePatientAllergiesResponseAllergiesItemVerificationStatus>,
}

impl ReplacePatientAllergiesResponseAllergiesItemBuilder {
    pub fn category(mut self, value: ReplacePatientAllergiesResponseAllergiesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(
        mut self,
        value: ReplacePatientAllergiesResponseAllergiesItemCodeSystem,
    ) -> Self {
        self.code_system = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reactions(
        mut self,
        value: Vec<ReplacePatientAllergiesResponseAllergiesItemReactionsItem>,
    ) -> Self {
        self.reactions = Some(value);
        self
    }

    pub fn severity(mut self, value: ReplacePatientAllergiesResponseAllergiesItemSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn source(mut self, value: ReplacePatientAllergiesResponseAllergiesItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn substance(mut self, value: impl Into<String>) -> Self {
        self.substance = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ReplacePatientAllergiesResponseAllergiesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn verification_status(
        mut self,
        value: ReplacePatientAllergiesResponseAllergiesItemVerificationStatus,
    ) -> Self {
        self.verification_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesResponseAllergiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](ReplacePatientAllergiesResponseAllergiesItemBuilder::category)
    /// - [`id`](ReplacePatientAllergiesResponseAllergiesItemBuilder::id)
    /// - [`reactions`](ReplacePatientAllergiesResponseAllergiesItemBuilder::reactions)
    /// - [`source`](ReplacePatientAllergiesResponseAllergiesItemBuilder::source)
    /// - [`substance`](ReplacePatientAllergiesResponseAllergiesItemBuilder::substance)
    /// - [`verification_status`](ReplacePatientAllergiesResponseAllergiesItemBuilder::verification_status)
    pub fn build(self) -> Result<ReplacePatientAllergiesResponseAllergiesItem, BuildError> {
        Ok(ReplacePatientAllergiesResponseAllergiesItem {
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            code: self.code,
            code_system: self.code_system,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
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

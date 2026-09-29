pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetPatientAllergiesResponseAllergiesItem {
    pub category: GetPatientAllergiesResponseAllergiesItemCategory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "codeSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_system: Option<GetPatientAllergiesResponseAllergiesItemCodeSystem>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reactions: Vec<GetPatientAllergiesResponseAllergiesItemReactionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<GetPatientAllergiesResponseAllergiesItemSeverity>,
    pub source: GetPatientAllergiesResponseAllergiesItemSource,
    #[serde(default)]
    pub substance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GetPatientAllergiesResponseAllergiesItemType>,
    #[serde(rename = "verificationStatus")]
    pub verification_status: GetPatientAllergiesResponseAllergiesItemVerificationStatus,
}

impl GetPatientAllergiesResponseAllergiesItem {
    pub fn builder() -> GetPatientAllergiesResponseAllergiesItemBuilder {
        <GetPatientAllergiesResponseAllergiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPatientAllergiesResponseAllergiesItemBuilder {
    category: Option<GetPatientAllergiesResponseAllergiesItemCategory>,
    code: Option<String>,
    code_system: Option<GetPatientAllergiesResponseAllergiesItemCodeSystem>,
    id: Option<String>,
    reactions: Option<Vec<GetPatientAllergiesResponseAllergiesItemReactionsItem>>,
    severity: Option<GetPatientAllergiesResponseAllergiesItemSeverity>,
    source: Option<GetPatientAllergiesResponseAllergiesItemSource>,
    substance: Option<String>,
    r#type: Option<GetPatientAllergiesResponseAllergiesItemType>,
    verification_status: Option<GetPatientAllergiesResponseAllergiesItemVerificationStatus>,
}

impl GetPatientAllergiesResponseAllergiesItemBuilder {
    pub fn category(mut self, value: GetPatientAllergiesResponseAllergiesItemCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn code_system(
        mut self,
        value: GetPatientAllergiesResponseAllergiesItemCodeSystem,
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
        value: Vec<GetPatientAllergiesResponseAllergiesItemReactionsItem>,
    ) -> Self {
        self.reactions = Some(value);
        self
    }

    pub fn severity(mut self, value: GetPatientAllergiesResponseAllergiesItemSeverity) -> Self {
        self.severity = Some(value);
        self
    }

    pub fn source(mut self, value: GetPatientAllergiesResponseAllergiesItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn substance(mut self, value: impl Into<String>) -> Self {
        self.substance = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: GetPatientAllergiesResponseAllergiesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn verification_status(
        mut self,
        value: GetPatientAllergiesResponseAllergiesItemVerificationStatus,
    ) -> Self {
        self.verification_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPatientAllergiesResponseAllergiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`category`](GetPatientAllergiesResponseAllergiesItemBuilder::category)
    /// - [`id`](GetPatientAllergiesResponseAllergiesItemBuilder::id)
    /// - [`reactions`](GetPatientAllergiesResponseAllergiesItemBuilder::reactions)
    /// - [`source`](GetPatientAllergiesResponseAllergiesItemBuilder::source)
    /// - [`substance`](GetPatientAllergiesResponseAllergiesItemBuilder::substance)
    /// - [`verification_status`](GetPatientAllergiesResponseAllergiesItemBuilder::verification_status)
    pub fn build(self) -> Result<GetPatientAllergiesResponseAllergiesItem, BuildError> {
        Ok(GetPatientAllergiesResponseAllergiesItem {
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

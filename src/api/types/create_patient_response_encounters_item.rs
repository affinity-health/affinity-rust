pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePatientResponseEncountersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    pub occurred_at: String,
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(default)]
    pub r#type: String,
}

impl CreatePatientResponseEncountersItem {
    pub fn builder() -> CreatePatientResponseEncountersItemBuilder {
        <CreatePatientResponseEncountersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientResponseEncountersItemBuilder {
    notes: Option<String>,
    occurred_at: Option<String>,
    provider_name: Option<String>,
    r#type: Option<String>,
}

impl CreatePatientResponseEncountersItemBuilder {
    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: impl Into<String>) -> Self {
        self.occurred_at = Some(value.into());
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientResponseEncountersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`occurred_at`](CreatePatientResponseEncountersItemBuilder::occurred_at)
    /// - [`r#type`](CreatePatientResponseEncountersItemBuilder::r#type)
    pub fn build(self) -> Result<CreatePatientResponseEncountersItem, BuildError> {
        Ok(CreatePatientResponseEncountersItem {
            notes: self.notes,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            provider_name: self.provider_name,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

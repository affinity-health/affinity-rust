pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientRequestEncountersItem {
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

impl UpdatePatientRequestEncountersItem {
    pub fn builder() -> UpdatePatientRequestEncountersItemBuilder {
        <UpdatePatientRequestEncountersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestEncountersItemBuilder {
    notes: Option<String>,
    occurred_at: Option<String>,
    provider_name: Option<String>,
    r#type: Option<String>,
}

impl UpdatePatientRequestEncountersItemBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePatientRequestEncountersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`occurred_at`](UpdatePatientRequestEncountersItemBuilder::occurred_at)
    /// - [`r#type`](UpdatePatientRequestEncountersItemBuilder::r#type)
    pub fn build(self) -> Result<UpdatePatientRequestEncountersItem, BuildError> {
        Ok(UpdatePatientRequestEncountersItem {
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

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePracticeRequestPrescribersItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(rename = "licenseStates")]
    #[serde(default)]
    pub license_states: Vec<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub npi: String,
}

impl CreatePracticeRequestPrescribersItem {
    pub fn builder() -> CreatePracticeRequestPrescribersItemBuilder {
        <CreatePracticeRequestPrescribersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePracticeRequestPrescribersItemBuilder {
    credentials: Option<String>,
    license_states: Option<Vec<String>>,
    name: Option<String>,
    npi: Option<String>,
}

impl CreatePracticeRequestPrescribersItemBuilder {
    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn license_states(mut self, value: Vec<String>) -> Self {
        self.license_states = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePracticeRequestPrescribersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`license_states`](CreatePracticeRequestPrescribersItemBuilder::license_states)
    /// - [`name`](CreatePracticeRequestPrescribersItemBuilder::name)
    /// - [`npi`](CreatePracticeRequestPrescribersItemBuilder::npi)
    pub fn build(self) -> Result<CreatePracticeRequestPrescribersItem, BuildError> {
        Ok(CreatePracticeRequestPrescribersItem {
            credentials: self.credentials,
            license_states: self
                .license_states
                .ok_or_else(|| BuildError::missing_field("license_states"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            npi: self.npi.ok_or_else(|| BuildError::missing_field("npi"))?,
        })
    }
}

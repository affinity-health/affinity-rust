pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<PreviewOrderRequestPrescriberProfile>,
}

impl PreviewOrderRequestPrescriber {
    pub fn builder() -> PreviewOrderRequestPrescriberBuilder {
        <PreviewOrderRequestPrescriberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriberBuilder {
    id: Option<String>,
    npi: Option<String>,
    external_id: Option<String>,
    profile: Option<PreviewOrderRequestPrescriberProfile>,
}

impl PreviewOrderRequestPrescriberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn profile(mut self, value: PreviewOrderRequestPrescriberProfile) -> Self {
        self.profile = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriber`].
    pub fn build(self) -> Result<PreviewOrderRequestPrescriber, BuildError> {
        Ok(PreviewOrderRequestPrescriber {
            id: self.id,
            npi: self.npi,
            external_id: self.external_id,
            profile: self.profile,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectOrderRequestPrescriber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<RejectOrderRequestPrescriberProfile>,
}

impl RejectOrderRequestPrescriber {
    pub fn builder() -> RejectOrderRequestPrescriberBuilder {
        <RejectOrderRequestPrescriberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectOrderRequestPrescriberBuilder {
    id: Option<String>,
    npi: Option<String>,
    external_id: Option<String>,
    profile: Option<RejectOrderRequestPrescriberProfile>,
}

impl RejectOrderRequestPrescriberBuilder {
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

    pub fn profile(mut self, value: RejectOrderRequestPrescriberProfile) -> Self {
        self.profile = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RejectOrderRequestPrescriber`].
    pub fn build(self) -> Result<RejectOrderRequestPrescriber, BuildError> {
        Ok(RejectOrderRequestPrescriber {
            id: self.id,
            npi: self.npi,
            external_id: self.external_id,
            profile: self.profile,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPrescriber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<CreateOrderRequestPrescriberProfile>,
}

impl CreateOrderRequestPrescriber {
    pub fn builder() -> CreateOrderRequestPrescriberBuilder {
        <CreateOrderRequestPrescriberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriberBuilder {
    id: Option<String>,
    npi: Option<String>,
    external_id: Option<String>,
    profile: Option<CreateOrderRequestPrescriberProfile>,
}

impl CreateOrderRequestPrescriberBuilder {
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

    pub fn profile(mut self, value: CreateOrderRequestPrescriberProfile) -> Self {
        self.profile = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriber`].
    pub fn build(self) -> Result<CreateOrderRequestPrescriber, BuildError> {
        Ok(CreateOrderRequestPrescriber {
            id: self.id,
            npi: self.npi,
            external_id: self.external_id,
            profile: self.profile,
        })
    }
}

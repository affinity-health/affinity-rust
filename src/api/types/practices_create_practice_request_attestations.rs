pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePracticeRequestAttestations {
    #[serde(rename = "authorizedPracticeRelationship")]
    #[serde(default)]
    pub authorized_practice_relationship: bool,
    #[serde(rename = "authorizedPhiTransfer")]
    #[serde(default)]
    pub authorized_phi_transfer: bool,
    #[serde(rename = "minimumNecessaryPhi")]
    #[serde(default)]
    pub minimum_necessary_phi: bool,
    #[serde(rename = "providerDataAccuracy")]
    #[serde(default)]
    pub provider_data_accuracy: bool,
}

impl CreatePracticeRequestAttestations {
    pub fn builder() -> CreatePracticeRequestAttestationsBuilder {
        <CreatePracticeRequestAttestationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePracticeRequestAttestationsBuilder {
    authorized_practice_relationship: Option<bool>,
    authorized_phi_transfer: Option<bool>,
    minimum_necessary_phi: Option<bool>,
    provider_data_accuracy: Option<bool>,
}

impl CreatePracticeRequestAttestationsBuilder {
    pub fn authorized_practice_relationship(mut self, value: bool) -> Self {
        self.authorized_practice_relationship = Some(value);
        self
    }

    pub fn authorized_phi_transfer(mut self, value: bool) -> Self {
        self.authorized_phi_transfer = Some(value);
        self
    }

    pub fn minimum_necessary_phi(mut self, value: bool) -> Self {
        self.minimum_necessary_phi = Some(value);
        self
    }

    pub fn provider_data_accuracy(mut self, value: bool) -> Self {
        self.provider_data_accuracy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePracticeRequestAttestations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`authorized_practice_relationship`](CreatePracticeRequestAttestationsBuilder::authorized_practice_relationship)
    /// - [`authorized_phi_transfer`](CreatePracticeRequestAttestationsBuilder::authorized_phi_transfer)
    /// - [`minimum_necessary_phi`](CreatePracticeRequestAttestationsBuilder::minimum_necessary_phi)
    /// - [`provider_data_accuracy`](CreatePracticeRequestAttestationsBuilder::provider_data_accuracy)
    pub fn build(self) -> Result<CreatePracticeRequestAttestations, BuildError> {
        Ok(CreatePracticeRequestAttestations {
            authorized_practice_relationship: self
                .authorized_practice_relationship
                .ok_or_else(|| BuildError::missing_field("authorized_practice_relationship"))?,
            authorized_phi_transfer: self
                .authorized_phi_transfer
                .ok_or_else(|| BuildError::missing_field("authorized_phi_transfer"))?,
            minimum_necessary_phi: self
                .minimum_necessary_phi
                .ok_or_else(|| BuildError::missing_field("minimum_necessary_phi"))?,
            provider_data_accuracy: self
                .provider_data_accuracy
                .ok_or_else(|| BuildError::missing_field("provider_data_accuracy"))?,
        })
    }
}

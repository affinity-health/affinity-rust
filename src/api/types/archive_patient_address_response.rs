pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ArchivePatientAddressResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub address: ArchivePatientAddressResponseAddress,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "preferredShipping")]
    #[serde(default)]
    pub preferred_shipping: bool,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[serde(rename = "archivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
}

impl ArchivePatientAddressResponse {
    pub fn builder() -> ArchivePatientAddressResponseBuilder {
        <ArchivePatientAddressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ArchivePatientAddressResponseBuilder {
    id: Option<String>,
    address: Option<ArchivePatientAddressResponseAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
    archived_at: Option<String>,
}

impl ArchivePatientAddressResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(mut self, value: ArchivePatientAddressResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn preferred_shipping(mut self, value: bool) -> Self {
        self.preferred_shipping = Some(value);
        self
    }

    pub fn recipient_name(mut self, value: impl Into<String>) -> Self {
        self.recipient_name = Some(value.into());
        self
    }

    pub fn archived_at(mut self, value: impl Into<String>) -> Self {
        self.archived_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ArchivePatientAddressResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ArchivePatientAddressResponseBuilder::id)
    /// - [`address`](ArchivePatientAddressResponseBuilder::address)
    /// - [`label`](ArchivePatientAddressResponseBuilder::label)
    /// - [`preferred_shipping`](ArchivePatientAddressResponseBuilder::preferred_shipping)
    pub fn build(self) -> Result<ArchivePatientAddressResponse, BuildError> {
        Ok(ArchivePatientAddressResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            preferred_shipping: self
                .preferred_shipping
                .ok_or_else(|| BuildError::missing_field("preferred_shipping"))?,
            recipient_name: self.recipient_name,
            archived_at: self.archived_at,
        })
    }
}

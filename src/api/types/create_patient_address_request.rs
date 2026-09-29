pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePatientAddressRequest {
    #[serde(default)]
    pub address: CreatePatientAddressRequestAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "preferredShipping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_shipping: Option<bool>,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
}

impl CreatePatientAddressRequest {
    pub fn builder() -> CreatePatientAddressRequestBuilder {
        <CreatePatientAddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientAddressRequestBuilder {
    address: Option<CreatePatientAddressRequestAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
}

impl CreatePatientAddressRequestBuilder {
    pub fn address(mut self, value: CreatePatientAddressRequestAddress) -> Self {
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

    /// Consumes the builder and constructs a [`CreatePatientAddressRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreatePatientAddressRequestBuilder::address)
    pub fn build(self) -> Result<CreatePatientAddressRequest, BuildError> {
        Ok(CreatePatientAddressRequest {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            label: self.label,
            preferred_shipping: self.preferred_shipping,
            recipient_name: self.recipient_name,
        })
    }
}

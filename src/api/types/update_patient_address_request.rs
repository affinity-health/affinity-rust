pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePatientAddressRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
    #[serde(rename = "preferredShipping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_shipping: Option<bool>,
}

impl UpdatePatientAddressRequest {
    pub fn builder() -> UpdatePatientAddressRequestBuilder {
        <UpdatePatientAddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientAddressRequestBuilder {
    address: Option<UpdatePatientAddressRequestAddress>,
    label: Option<String>,
    recipient_name: Option<String>,
    preferred_shipping: Option<bool>,
}

impl UpdatePatientAddressRequestBuilder {
    pub fn address(mut self, value: UpdatePatientAddressRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn recipient_name(mut self, value: impl Into<String>) -> Self {
        self.recipient_name = Some(value.into());
        self
    }

    pub fn preferred_shipping(mut self, value: bool) -> Self {
        self.preferred_shipping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientAddressRequest`].
    pub fn build(self) -> Result<UpdatePatientAddressRequest, BuildError> {
        Ok(UpdatePatientAddressRequest {
            address: self.address,
            label: self.label,
            recipient_name: self.recipient_name,
            preferred_shipping: self.preferred_shipping,
        })
    }
}

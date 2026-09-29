pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePatientRequestAddressesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub address: CreatePatientRequestAddressesItemAddress,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "preferredShipping")]
    #[serde(default)]
    pub preferred_shipping: bool,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
}

impl CreatePatientRequestAddressesItem {
    pub fn builder() -> CreatePatientRequestAddressesItemBuilder {
        <CreatePatientRequestAddressesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientRequestAddressesItemBuilder {
    id: Option<String>,
    address: Option<CreatePatientRequestAddressesItemAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
}

impl CreatePatientRequestAddressesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(mut self, value: CreatePatientRequestAddressesItemAddress) -> Self {
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

    /// Consumes the builder and constructs a [`CreatePatientRequestAddressesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreatePatientRequestAddressesItemBuilder::address)
    /// - [`label`](CreatePatientRequestAddressesItemBuilder::label)
    /// - [`preferred_shipping`](CreatePatientRequestAddressesItemBuilder::preferred_shipping)
    pub fn build(self) -> Result<CreatePatientRequestAddressesItem, BuildError> {
        Ok(CreatePatientRequestAddressesItem {
            id: self.id,
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
        })
    }
}

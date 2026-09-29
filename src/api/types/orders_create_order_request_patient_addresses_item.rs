pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPatientAddressesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub address: CreateOrderRequestPatientAddressesItemAddress,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "preferredShipping")]
    #[serde(default)]
    pub preferred_shipping: bool,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
}

impl CreateOrderRequestPatientAddressesItem {
    pub fn builder() -> CreateOrderRequestPatientAddressesItemBuilder {
        <CreateOrderRequestPatientAddressesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPatientAddressesItemBuilder {
    id: Option<String>,
    address: Option<CreateOrderRequestPatientAddressesItemAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
}

impl CreateOrderRequestPatientAddressesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(mut self, value: CreateOrderRequestPatientAddressesItemAddress) -> Self {
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

    /// Consumes the builder and constructs a [`CreateOrderRequestPatientAddressesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreateOrderRequestPatientAddressesItemBuilder::address)
    /// - [`label`](CreateOrderRequestPatientAddressesItemBuilder::label)
    /// - [`preferred_shipping`](CreateOrderRequestPatientAddressesItemBuilder::preferred_shipping)
    pub fn build(self) -> Result<CreateOrderRequestPatientAddressesItem, BuildError> {
        Ok(CreateOrderRequestPatientAddressesItem {
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

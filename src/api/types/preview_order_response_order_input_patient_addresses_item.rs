pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseOrderInputPatientAddressesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub address: PreviewOrderResponseOrderInputPatientAddressesItemAddress,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "preferredShipping")]
    #[serde(default)]
    pub preferred_shipping: bool,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
}

impl PreviewOrderResponseOrderInputPatientAddressesItem {
    pub fn builder() -> PreviewOrderResponseOrderInputPatientAddressesItemBuilder {
        <PreviewOrderResponseOrderInputPatientAddressesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPatientAddressesItemBuilder {
    id: Option<String>,
    address: Option<PreviewOrderResponseOrderInputPatientAddressesItemAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
}

impl PreviewOrderResponseOrderInputPatientAddressesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(
        mut self,
        value: PreviewOrderResponseOrderInputPatientAddressesItemAddress,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPatientAddressesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](PreviewOrderResponseOrderInputPatientAddressesItemBuilder::address)
    /// - [`label`](PreviewOrderResponseOrderInputPatientAddressesItemBuilder::label)
    /// - [`preferred_shipping`](PreviewOrderResponseOrderInputPatientAddressesItemBuilder::preferred_shipping)
    pub fn build(self) -> Result<PreviewOrderResponseOrderInputPatientAddressesItem, BuildError> {
        Ok(PreviewOrderResponseOrderInputPatientAddressesItem {
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

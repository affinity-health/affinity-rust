pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestOrdersItemPatientAddressesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub address: CreateOrderBatchRequestOrdersItemPatientAddressesItemAddress,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "preferredShipping")]
    #[serde(default)]
    pub preferred_shipping: bool,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_name: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPatientAddressesItem {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder {
        <CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder {
    id: Option<String>,
    address: Option<CreateOrderBatchRequestOrdersItemPatientAddressesItemAddress>,
    label: Option<String>,
    preferred_shipping: Option<bool>,
    recipient_name: Option<String>,
}

impl CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn address(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPatientAddressesItemAddress,
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

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPatientAddressesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder::address)
    /// - [`label`](CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder::label)
    /// - [`preferred_shipping`](CreateOrderBatchRequestOrdersItemPatientAddressesItemBuilder::preferred_shipping)
    pub fn build(
        self,
    ) -> Result<CreateOrderBatchRequestOrdersItemPatientAddressesItem, BuildError> {
        Ok(CreateOrderBatchRequestOrdersItemPatientAddressesItem {
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

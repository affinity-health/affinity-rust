pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPatientAddressesItemAddress {
    #[serde(default)]
    pub city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CreateOrderRequestPatientAddressesItemAddressCountry>,
    #[serde(default)]
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub state: String,
}

impl CreateOrderRequestPatientAddressesItemAddress {
    pub fn builder() -> CreateOrderRequestPatientAddressesItemAddressBuilder {
        <CreateOrderRequestPatientAddressesItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPatientAddressesItemAddressBuilder {
    city: Option<String>,
    country: Option<CreateOrderRequestPatientAddressesItemAddressCountry>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CreateOrderRequestPatientAddressesItemAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: CreateOrderRequestPatientAddressesItemAddressCountry) -> Self {
        self.country = Some(value);
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPatientAddressesItemAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](CreateOrderRequestPatientAddressesItemAddressBuilder::city)
    /// - [`line1`](CreateOrderRequestPatientAddressesItemAddressBuilder::line1)
    /// - [`postal_code`](CreateOrderRequestPatientAddressesItemAddressBuilder::postal_code)
    /// - [`state`](CreateOrderRequestPatientAddressesItemAddressBuilder::state)
    pub fn build(self) -> Result<CreateOrderRequestPatientAddressesItemAddress, BuildError> {
        Ok(CreateOrderRequestPatientAddressesItemAddress {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self.country,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePatientAddressRequestAddress {
    #[serde(default)]
    pub city: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<UpdatePatientAddressRequestAddressCountry>,
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

impl UpdatePatientAddressRequestAddress {
    pub fn builder() -> UpdatePatientAddressRequestAddressBuilder {
        <UpdatePatientAddressRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientAddressRequestAddressBuilder {
    city: Option<String>,
    country: Option<UpdatePatientAddressRequestAddressCountry>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl UpdatePatientAddressRequestAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: UpdatePatientAddressRequestAddressCountry) -> Self {
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

    /// Consumes the builder and constructs a [`UpdatePatientAddressRequestAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](UpdatePatientAddressRequestAddressBuilder::city)
    /// - [`line1`](UpdatePatientAddressRequestAddressBuilder::line1)
    /// - [`postal_code`](UpdatePatientAddressRequestAddressBuilder::postal_code)
    /// - [`state`](UpdatePatientAddressRequestAddressBuilder::state)
    pub fn build(self) -> Result<UpdatePatientAddressRequestAddress, BuildError> {
        Ok(UpdatePatientAddressRequestAddress {
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

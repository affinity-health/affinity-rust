pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamPrescriberResponseAddress {
    #[serde(default)]
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub country: String,
}

impl GetPracticeTeamPrescriberResponseAddress {
    pub fn builder() -> GetPracticeTeamPrescriberResponseAddressBuilder {
        <GetPracticeTeamPrescriberResponseAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamPrescriberResponseAddressBuilder {
    line1: Option<String>,
    line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl GetPracticeTeamPrescriberResponseAddressBuilder {
    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamPrescriberResponseAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line1`](GetPracticeTeamPrescriberResponseAddressBuilder::line1)
    /// - [`city`](GetPracticeTeamPrescriberResponseAddressBuilder::city)
    /// - [`state`](GetPracticeTeamPrescriberResponseAddressBuilder::state)
    /// - [`postal_code`](GetPracticeTeamPrescriberResponseAddressBuilder::postal_code)
    /// - [`country`](GetPracticeTeamPrescriberResponseAddressBuilder::country)
    pub fn build(self) -> Result<GetPracticeTeamPrescriberResponseAddress, BuildError> {
        Ok(GetPracticeTeamPrescriberResponseAddress {
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
        })
    }
}

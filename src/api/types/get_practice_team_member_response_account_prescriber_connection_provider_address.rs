pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress {
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

impl GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress {
    pub fn builder(
    ) -> GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder {
        <GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder {
    line1: Option<String>,
    line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder {
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

    /// Consumes the builder and constructs a [`GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line1`](GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder::line1)
    /// - [`city`](GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder::city)
    /// - [`state`](GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder::state)
    /// - [`postal_code`](GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder::postal_code)
    /// - [`country`](GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddressBuilder::country)
    pub fn build(
        self,
    ) -> Result<GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress, BuildError>
    {
        Ok(
            GetPracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress {
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
            },
        )
    }
}

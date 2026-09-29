pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddress
{
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

impl ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddress {
    pub fn builder() -> ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder{
        <ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder
{
    line1: Option<String>,
    line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder {
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

    /// Consumes the builder and constructs a [`ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line1`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder::line1)
    /// - [`city`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder::city)
    /// - [`state`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder::state)
    /// - [`postal_code`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder::postal_code)
    /// - [`country`](ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddressBuilder::country)
    pub fn build(self) -> Result<ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddress, BuildError> {
        Ok(ListPracticeTeamInvitationsResponseDataItemPersonAccountPrescriberConnectionProviderAddress {
            line1: self.line1.ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            postal_code: self.postal_code.ok_or_else(|| BuildError::missing_field("postal_code"))?,
            country: self.country.ok_or_else(|| BuildError::missing_field("country"))?,
        })
    }
}

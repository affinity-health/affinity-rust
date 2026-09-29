pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequestProfileDetailsAddressesItem {
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub line1: String,
    #[serde(default)]
    pub line2: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub fax: String,
}

impl InvitePracticeTeamPersonRequestProfileDetailsAddressesItem {
    pub fn builder() -> InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder {
        <InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder {
    purpose: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    fax: Option<String>,
}

impl InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder {
    pub fn purpose(mut self, value: impl Into<String>) -> Self {
        self.purpose = Some(value.into());
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn fax(mut self, value: impl Into<String>) -> Self {
        self.fax = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequestProfileDetailsAddressesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`purpose`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::purpose)
    /// - [`line1`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::line1)
    /// - [`line2`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::line2)
    /// - [`city`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::city)
    /// - [`state`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::state)
    /// - [`postal_code`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::postal_code)
    /// - [`country`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::country)
    /// - [`phone`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::phone)
    /// - [`fax`](InvitePracticeTeamPersonRequestProfileDetailsAddressesItemBuilder::fax)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonRequestProfileDetailsAddressesItem, BuildError> {
        Ok(InvitePracticeTeamPersonRequestProfileDetailsAddressesItem {
            purpose: self
                .purpose
                .ok_or_else(|| BuildError::missing_field("purpose"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self
                .line2
                .ok_or_else(|| BuildError::missing_field("line2"))?,
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
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            fax: self.fax.ok_or_else(|| BuildError::missing_field("fax"))?,
        })
    }
}

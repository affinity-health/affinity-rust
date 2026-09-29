pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub state: String,
    #[serde(rename = "licenseNumber")]
    #[serde(default)]
    pub license_number: String,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItem {
    pub fn builder(
    ) -> GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder
    {
        <GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder
{
    id: Option<String>,
    state: Option<String>,
    license_number: Option<String>,
    expires_at: Option<String>,
}

impl GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn license_number(mut self, value: impl Into<String>) -> Self {
        self.license_number = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder::id)
    /// - [`state`](GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder::state)
    /// - [`license_number`](GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItemBuilder::license_number)
    pub fn build(
        self,
    ) -> Result<
        GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItem,
        BuildError,
    > {
        Ok(GetPracticeTeamInvitationResponsePersonAccountPrescriberConnectionProviderLicensesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            license_number: self.license_number.ok_or_else(|| BuildError::missing_field("license_number"))?,
            expires_at: self.expires_at,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequest {
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<InvitePracticeTeamPersonRequestRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<InvitePracticeTeamPersonRequestRolesItem>>,
    #[serde(rename = "profileDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_details: Option<InvitePracticeTeamPersonRequestProfileDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<InvitePracticeTeamPersonRequestLicensesItem>>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<InvitePracticeTeamPersonRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "locationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
}

impl InvitePracticeTeamPersonRequest {
    pub fn builder() -> InvitePracticeTeamPersonRequestBuilder {
        <InvitePracticeTeamPersonRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestBuilder {
    external_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    role: Option<InvitePracticeTeamPersonRequestRole>,
    roles: Option<Vec<InvitePracticeTeamPersonRequestRolesItem>>,
    profile_details: Option<InvitePracticeTeamPersonRequestProfileDetails>,
    npi: Option<String>,
    licenses: Option<Vec<InvitePracticeTeamPersonRequestLicensesItem>>,
    legal_name: Option<String>,
    display_name: Option<String>,
    credentials: Option<String>,
    address: Option<InvitePracticeTeamPersonRequestAddress>,
    phone: Option<String>,
    location_ids: Option<Vec<String>>,
}

impl InvitePracticeTeamPersonRequestBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn role(mut self, value: InvitePracticeTeamPersonRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<InvitePracticeTeamPersonRequestRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn profile_details(mut self, value: InvitePracticeTeamPersonRequestProfileDetails) -> Self {
        self.profile_details = Some(value);
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn licenses(mut self, value: Vec<InvitePracticeTeamPersonRequestLicensesItem>) -> Self {
        self.licenses = Some(value);
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn address(mut self, value: InvitePracticeTeamPersonRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](InvitePracticeTeamPersonRequestBuilder::external_id)
    /// - [`email`](InvitePracticeTeamPersonRequestBuilder::email)
    /// - [`name`](InvitePracticeTeamPersonRequestBuilder::name)
    pub fn build(self) -> Result<InvitePracticeTeamPersonRequest, BuildError> {
        Ok(InvitePracticeTeamPersonRequest {
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role,
            roles: self.roles,
            profile_details: self.profile_details,
            npi: self.npi,
            licenses: self.licenses,
            legal_name: self.legal_name,
            display_name: self.display_name,
            credentials: self.credentials,
            address: self.address,
            phone: self.phone,
            location_ids: self.location_ids,
        })
    }
}

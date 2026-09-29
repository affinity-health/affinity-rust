pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterUserRequest {
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    pub role: RegisterUserRequestRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RegisterUserRequestRolesItem>>,
    #[serde(rename = "profileDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_details: Option<RegisterUserRequestProfileDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<RegisterUserRequestLicensesItem>>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<RegisterUserRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "locationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    #[serde(rename = "identityAttestation")]
    #[serde(default)]
    pub identity_attestation: bool,
}

impl RegisterUserRequest {
    pub fn builder() -> RegisterUserRequestBuilder {
        <RegisterUserRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestBuilder {
    external_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    role: Option<RegisterUserRequestRole>,
    roles: Option<Vec<RegisterUserRequestRolesItem>>,
    profile_details: Option<RegisterUserRequestProfileDetails>,
    npi: Option<String>,
    licenses: Option<Vec<RegisterUserRequestLicensesItem>>,
    legal_name: Option<String>,
    display_name: Option<String>,
    credentials: Option<String>,
    address: Option<RegisterUserRequestAddress>,
    phone: Option<String>,
    location_ids: Option<Vec<String>>,
    identity_attestation: Option<bool>,
}

impl RegisterUserRequestBuilder {
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

    pub fn role(mut self, value: RegisterUserRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<RegisterUserRequestRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn profile_details(mut self, value: RegisterUserRequestProfileDetails) -> Self {
        self.profile_details = Some(value);
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn licenses(mut self, value: Vec<RegisterUserRequestLicensesItem>) -> Self {
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

    pub fn address(mut self, value: RegisterUserRequestAddress) -> Self {
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

    pub fn identity_attestation(mut self, value: bool) -> Self {
        self.identity_attestation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](RegisterUserRequestBuilder::external_id)
    /// - [`email`](RegisterUserRequestBuilder::email)
    /// - [`name`](RegisterUserRequestBuilder::name)
    /// - [`role`](RegisterUserRequestBuilder::role)
    /// - [`identity_attestation`](RegisterUserRequestBuilder::identity_attestation)
    pub fn build(self) -> Result<RegisterUserRequest, BuildError> {
        Ok(RegisterUserRequest {
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
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
            identity_attestation: self
                .identity_attestation
                .ok_or_else(|| BuildError::missing_field("identity_attestation"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "legalName")]
    #[serde(default)]
    pub legal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress>,
    #[serde(default)]
    pub npi: String,
    #[serde(rename = "practiceStatus")]
    #[serde(default)]
    pub practice_status: String,
    #[serde(default)]
    pub licenses:
        Vec<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderLicensesItem>,
}

impl UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider {
    pub fn builder() -> UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder {
        <UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder {
    id: Option<String>,
    name: Option<String>,
    legal_name: Option<String>,
    credentials: Option<String>,
    phone: Option<String>,
    address: Option<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress>,
    npi: Option<String>,
    practice_status: Option<String>,
    licenses: Option<
        Vec<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderLicensesItem>,
    >,
}

impl UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn address(
        mut self,
        value: UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderAddress,
    ) -> Self {
        self.address = Some(value);
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn practice_status(mut self, value: impl Into<String>) -> Self {
        self.practice_status = Some(value.into());
        self
    }

    pub fn licenses(
        mut self,
        value: Vec<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderLicensesItem>,
    ) -> Self {
        self.licenses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::id)
    /// - [`name`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::name)
    /// - [`legal_name`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::legal_name)
    /// - [`npi`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::npi)
    /// - [`practice_status`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::practice_status)
    /// - [`licenses`](UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProviderBuilder::licenses)
    pub fn build(
        self,
    ) -> Result<UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider, BuildError>
    {
        Ok(
            UpdatePracticeTeamMemberResponseAccountPrescriberConnectionProvider {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                legal_name: self
                    .legal_name
                    .ok_or_else(|| BuildError::missing_field("legal_name"))?,
                credentials: self.credentials,
                phone: self.phone,
                address: self.address,
                npi: self.npi.ok_or_else(|| BuildError::missing_field("npi"))?,
                practice_status: self
                    .practice_status
                    .ok_or_else(|| BuildError::missing_field("practice_status"))?,
                licenses: self
                    .licenses
                    .ok_or_else(|| BuildError::missing_field("licenses"))?,
            },
        )
    }
}

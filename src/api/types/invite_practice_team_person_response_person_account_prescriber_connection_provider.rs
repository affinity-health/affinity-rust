pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider {
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
    pub address:
        Option<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderAddress>,
    #[serde(default)]
    pub npi: String,
    #[serde(rename = "practiceStatus")]
    #[serde(default)]
    pub practice_status: String,
    #[serde(default)]
    pub licenses:
        Vec<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderLicensesItem>,
}

impl InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider {
    pub fn builder(
    ) -> InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder {
        <InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder {
    id: Option<String>,
    name: Option<String>,
    legal_name: Option<String>,
    credentials: Option<String>,
    phone: Option<String>,
    address:
        Option<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderAddress>,
    npi: Option<String>,
    practice_status: Option<String>,
    licenses: Option<
        Vec<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderLicensesItem>,
    >,
}

impl InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder {
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
        value: InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderAddress,
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
        value: Vec<
            InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderLicensesItem,
        >,
    ) -> Self {
        self.licenses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::id)
    /// - [`name`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::name)
    /// - [`legal_name`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::legal_name)
    /// - [`npi`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::npi)
    /// - [`practice_status`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::practice_status)
    /// - [`licenses`](InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProviderBuilder::licenses)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider, BuildError>
    {
        Ok(
            InvitePracticeTeamPersonResponsePersonAccountPrescriberConnectionProvider {
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

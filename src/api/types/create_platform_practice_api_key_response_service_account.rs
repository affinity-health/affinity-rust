pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePlatformPracticeApiKeyResponseServiceAccount {
    #[serde(rename = "apiVersion")]
    pub api_version: CreatePlatformPracticeApiKeyResponseServiceAccountApiVersion,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "maxScopes")]
    #[serde(default)]
    pub max_scopes: Vec<CreatePlatformPracticeApiKeyResponseServiceAccountMaxScopesItem>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    pub status: CreatePlatformPracticeApiKeyResponseServiceAccountStatus,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    #[serde(rename = "subjectType")]
    pub subject_type: CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType,
}

impl CreatePlatformPracticeApiKeyResponseServiceAccount {
    pub fn builder() -> CreatePlatformPracticeApiKeyResponseServiceAccountBuilder {
        <CreatePlatformPracticeApiKeyResponseServiceAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlatformPracticeApiKeyResponseServiceAccountBuilder {
    api_version: Option<CreatePlatformPracticeApiKeyResponseServiceAccountApiVersion>,
    display_name: Option<String>,
    id: Option<String>,
    max_scopes: Option<Vec<CreatePlatformPracticeApiKeyResponseServiceAccountMaxScopesItem>>,
    organization_id: Option<String>,
    status: Option<CreatePlatformPracticeApiKeyResponseServiceAccountStatus>,
    subject_id: Option<String>,
    subject_type: Option<CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType>,
}

impl CreatePlatformPracticeApiKeyResponseServiceAccountBuilder {
    pub fn api_version(
        mut self,
        value: CreatePlatformPracticeApiKeyResponseServiceAccountApiVersion,
    ) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn max_scopes(
        mut self,
        value: Vec<CreatePlatformPracticeApiKeyResponseServiceAccountMaxScopesItem>,
    ) -> Self {
        self.max_scopes = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: CreatePlatformPracticeApiKeyResponseServiceAccountStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn subject_type(
        mut self,
        value: CreatePlatformPracticeApiKeyResponseServiceAccountSubjectType,
    ) -> Self {
        self.subject_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePlatformPracticeApiKeyResponseServiceAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::api_version)
    /// - [`display_name`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::display_name)
    /// - [`id`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::id)
    /// - [`max_scopes`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::max_scopes)
    /// - [`organization_id`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::organization_id)
    /// - [`status`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::status)
    /// - [`subject_id`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::subject_id)
    /// - [`subject_type`](CreatePlatformPracticeApiKeyResponseServiceAccountBuilder::subject_type)
    pub fn build(self) -> Result<CreatePlatformPracticeApiKeyResponseServiceAccount, BuildError> {
        Ok(CreatePlatformPracticeApiKeyResponseServiceAccount {
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_scopes: self
                .max_scopes
                .ok_or_else(|| BuildError::missing_field("max_scopes"))?,
            organization_id: self
                .organization_id
                .ok_or_else(|| BuildError::missing_field("organization_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subject_id: self
                .subject_id
                .ok_or_else(|| BuildError::missing_field("subject_id"))?,
            subject_type: self
                .subject_type
                .ok_or_else(|| BuildError::missing_field("subject_type"))?,
        })
    }
}

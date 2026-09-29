pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetApiAccessResponseServiceAccount {
    #[serde(rename = "apiVersion")]
    pub api_version: GetApiAccessResponseServiceAccountApiVersion,
    #[serde(default)]
    pub id: String,
    pub object: GetApiAccessResponseServiceAccountObject,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: String,
    #[serde(rename = "subjectType")]
    #[serde(default)]
    pub subject_type: String,
}

impl GetApiAccessResponseServiceAccount {
    pub fn builder() -> GetApiAccessResponseServiceAccountBuilder {
        <GetApiAccessResponseServiceAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetApiAccessResponseServiceAccountBuilder {
    api_version: Option<GetApiAccessResponseServiceAccountApiVersion>,
    id: Option<String>,
    object: Option<GetApiAccessResponseServiceAccountObject>,
    subject_id: Option<String>,
    subject_type: Option<String>,
}

impl GetApiAccessResponseServiceAccountBuilder {
    pub fn api_version(mut self, value: GetApiAccessResponseServiceAccountApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: GetApiAccessResponseServiceAccountObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn subject_id(mut self, value: impl Into<String>) -> Self {
        self.subject_id = Some(value.into());
        self
    }

    pub fn subject_type(mut self, value: impl Into<String>) -> Self {
        self.subject_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetApiAccessResponseServiceAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](GetApiAccessResponseServiceAccountBuilder::api_version)
    /// - [`id`](GetApiAccessResponseServiceAccountBuilder::id)
    /// - [`object`](GetApiAccessResponseServiceAccountBuilder::object)
    /// - [`subject_id`](GetApiAccessResponseServiceAccountBuilder::subject_id)
    /// - [`subject_type`](GetApiAccessResponseServiceAccountBuilder::subject_type)
    pub fn build(self) -> Result<GetApiAccessResponseServiceAccount, BuildError> {
        Ok(GetApiAccessResponseServiceAccount {
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            subject_id: self
                .subject_id
                .ok_or_else(|| BuildError::missing_field("subject_id"))?,
            subject_type: self
                .subject_type
                .ok_or_else(|| BuildError::missing_field("subject_type"))?,
        })
    }
}

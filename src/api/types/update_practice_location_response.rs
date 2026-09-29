pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePracticeLocationResponse {
    #[serde(default)]
    pub id: String,
    pub object: UpdatePracticeLocationResponseObject,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default)]
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub status: UpdatePracticeLocationResponseStatus,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl UpdatePracticeLocationResponse {
    pub fn builder() -> UpdatePracticeLocationResponseBuilder {
        <UpdatePracticeLocationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeLocationResponseBuilder {
    id: Option<String>,
    object: Option<UpdatePracticeLocationResponseObject>,
    practice_id: Option<String>,
    name: Option<String>,
    timezone: Option<String>,
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    phone: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
    status: Option<UpdatePracticeLocationResponseStatus>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl UpdatePracticeLocationResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: UpdatePracticeLocationResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdatePracticeLocationResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeLocationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdatePracticeLocationResponseBuilder::id)
    /// - [`object`](UpdatePracticeLocationResponseBuilder::object)
    /// - [`practice_id`](UpdatePracticeLocationResponseBuilder::practice_id)
    /// - [`name`](UpdatePracticeLocationResponseBuilder::name)
    /// - [`country`](UpdatePracticeLocationResponseBuilder::country)
    /// - [`status`](UpdatePracticeLocationResponseBuilder::status)
    /// - [`created_at`](UpdatePracticeLocationResponseBuilder::created_at)
    /// - [`updated_at`](UpdatePracticeLocationResponseBuilder::updated_at)
    pub fn build(self) -> Result<UpdatePracticeLocationResponse, BuildError> {
        Ok(UpdatePracticeLocationResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            timezone: self.timezone,
            city: self.city,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            line1: self.line1,
            line2: self.line2,
            phone: self.phone,
            postal_code: self.postal_code,
            state: self.state,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

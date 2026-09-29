pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RegisterUserResponse {
    pub object: RegisterUserResponseObject,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "memberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "prescriberId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_id: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(default)]
    pub external_id: String,
    #[serde(default)]
    pub livemode: bool,
}

impl RegisterUserResponse {
    pub fn builder() -> RegisterUserResponseBuilder {
        <RegisterUserResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserResponseBuilder {
    object: Option<RegisterUserResponseObject>,
    id: Option<String>,
    practice_id: Option<String>,
    member_id: Option<String>,
    prescriber_id: Option<String>,
    external_id: Option<String>,
    livemode: Option<bool>,
}

impl RegisterUserResponseBuilder {
    pub fn object(mut self, value: RegisterUserResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn prescriber_id(mut self, value: impl Into<String>) -> Self {
        self.prescriber_id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](RegisterUserResponseBuilder::object)
    /// - [`id`](RegisterUserResponseBuilder::id)
    /// - [`practice_id`](RegisterUserResponseBuilder::practice_id)
    /// - [`member_id`](RegisterUserResponseBuilder::member_id)
    /// - [`external_id`](RegisterUserResponseBuilder::external_id)
    /// - [`livemode`](RegisterUserResponseBuilder::livemode)
    pub fn build(self) -> Result<RegisterUserResponse, BuildError> {
        Ok(RegisterUserResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            member_id: self
                .member_id
                .ok_or_else(|| BuildError::missing_field("member_id"))?,
            prescriber_id: self.prescriber_id,
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
        })
    }
}

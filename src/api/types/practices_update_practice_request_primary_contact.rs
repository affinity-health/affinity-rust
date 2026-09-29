pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeRequestPrimaryContact {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UpdatePracticeRequestPrimaryContact {
    pub fn builder() -> UpdatePracticeRequestPrimaryContactBuilder {
        <UpdatePracticeRequestPrimaryContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeRequestPrimaryContactBuilder {
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl UpdatePracticeRequestPrimaryContactBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeRequestPrimaryContact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UpdatePracticeRequestPrimaryContactBuilder::email)
    /// - [`name`](UpdatePracticeRequestPrimaryContactBuilder::name)
    pub fn build(self) -> Result<UpdatePracticeRequestPrimaryContact, BuildError> {
        Ok(UpdatePracticeRequestPrimaryContact {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
        })
    }
}

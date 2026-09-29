pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeResponseContactsPrimary {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UpdatePracticeResponseContactsPrimary {
    pub fn builder() -> UpdatePracticeResponseContactsPrimaryBuilder {
        <UpdatePracticeResponseContactsPrimaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeResponseContactsPrimaryBuilder {
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl UpdatePracticeResponseContactsPrimaryBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePracticeResponseContactsPrimary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UpdatePracticeResponseContactsPrimaryBuilder::email)
    /// - [`name`](UpdatePracticeResponseContactsPrimaryBuilder::name)
    pub fn build(self) -> Result<UpdatePracticeResponseContactsPrimary, BuildError> {
        Ok(UpdatePracticeResponseContactsPrimary {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
        })
    }
}

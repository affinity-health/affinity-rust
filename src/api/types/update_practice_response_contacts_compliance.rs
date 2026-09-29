pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeResponseContactsCompliance {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl UpdatePracticeResponseContactsCompliance {
    pub fn builder() -> UpdatePracticeResponseContactsComplianceBuilder {
        <UpdatePracticeResponseContactsComplianceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeResponseContactsComplianceBuilder {
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl UpdatePracticeResponseContactsComplianceBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePracticeResponseContactsCompliance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UpdatePracticeResponseContactsComplianceBuilder::email)
    /// - [`name`](UpdatePracticeResponseContactsComplianceBuilder::name)
    pub fn build(self) -> Result<UpdatePracticeResponseContactsCompliance, BuildError> {
        Ok(UpdatePracticeResponseContactsCompliance {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeResponseContactsCompliance {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl GetPracticeResponseContactsCompliance {
    pub fn builder() -> GetPracticeResponseContactsComplianceBuilder {
        <GetPracticeResponseContactsComplianceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeResponseContactsComplianceBuilder {
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl GetPracticeResponseContactsComplianceBuilder {
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

    /// Consumes the builder and constructs a [`GetPracticeResponseContactsCompliance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](GetPracticeResponseContactsComplianceBuilder::email)
    /// - [`name`](GetPracticeResponseContactsComplianceBuilder::name)
    pub fn build(self) -> Result<GetPracticeResponseContactsCompliance, BuildError> {
        Ok(GetPracticeResponseContactsCompliance {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
        })
    }
}

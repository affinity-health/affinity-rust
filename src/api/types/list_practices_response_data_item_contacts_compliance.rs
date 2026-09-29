pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticesResponseDataItemContactsCompliance {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl ListPracticesResponseDataItemContactsCompliance {
    pub fn builder() -> ListPracticesResponseDataItemContactsComplianceBuilder {
        <ListPracticesResponseDataItemContactsComplianceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticesResponseDataItemContactsComplianceBuilder {
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl ListPracticesResponseDataItemContactsComplianceBuilder {
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

    /// Consumes the builder and constructs a [`ListPracticesResponseDataItemContactsCompliance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ListPracticesResponseDataItemContactsComplianceBuilder::email)
    /// - [`name`](ListPracticesResponseDataItemContactsComplianceBuilder::name)
    pub fn build(self) -> Result<ListPracticesResponseDataItemContactsCompliance, BuildError> {
        Ok(ListPracticesResponseDataItemContactsCompliance {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
        })
    }
}

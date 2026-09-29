pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePracticeResponseContacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<CreatePracticeResponseContactsCompliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<CreatePracticeResponseContactsPrimary>,
}

impl CreatePracticeResponseContacts {
    pub fn builder() -> CreatePracticeResponseContactsBuilder {
        <CreatePracticeResponseContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePracticeResponseContactsBuilder {
    compliance: Option<CreatePracticeResponseContactsCompliance>,
    primary: Option<CreatePracticeResponseContactsPrimary>,
}

impl CreatePracticeResponseContactsBuilder {
    pub fn compliance(mut self, value: CreatePracticeResponseContactsCompliance) -> Self {
        self.compliance = Some(value);
        self
    }

    pub fn primary(mut self, value: CreatePracticeResponseContactsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePracticeResponseContacts`].
    pub fn build(self) -> Result<CreatePracticeResponseContacts, BuildError> {
        Ok(CreatePracticeResponseContacts {
            compliance: self.compliance,
            primary: self.primary,
        })
    }
}

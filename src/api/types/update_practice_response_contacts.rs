pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePracticeResponseContacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<UpdatePracticeResponseContactsCompliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<UpdatePracticeResponseContactsPrimary>,
}

impl UpdatePracticeResponseContacts {
    pub fn builder() -> UpdatePracticeResponseContactsBuilder {
        <UpdatePracticeResponseContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeResponseContactsBuilder {
    compliance: Option<UpdatePracticeResponseContactsCompliance>,
    primary: Option<UpdatePracticeResponseContactsPrimary>,
}

impl UpdatePracticeResponseContactsBuilder {
    pub fn compliance(mut self, value: UpdatePracticeResponseContactsCompliance) -> Self {
        self.compliance = Some(value);
        self
    }

    pub fn primary(mut self, value: UpdatePracticeResponseContactsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePracticeResponseContacts`].
    pub fn build(self) -> Result<UpdatePracticeResponseContacts, BuildError> {
        Ok(UpdatePracticeResponseContacts {
            compliance: self.compliance,
            primary: self.primary,
        })
    }
}

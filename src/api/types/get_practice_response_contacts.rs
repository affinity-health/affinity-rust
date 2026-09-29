pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetPracticeResponseContacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<GetPracticeResponseContactsCompliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<GetPracticeResponseContactsPrimary>,
}

impl GetPracticeResponseContacts {
    pub fn builder() -> GetPracticeResponseContactsBuilder {
        <GetPracticeResponseContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPracticeResponseContactsBuilder {
    compliance: Option<GetPracticeResponseContactsCompliance>,
    primary: Option<GetPracticeResponseContactsPrimary>,
}

impl GetPracticeResponseContactsBuilder {
    pub fn compliance(mut self, value: GetPracticeResponseContactsCompliance) -> Self {
        self.compliance = Some(value);
        self
    }

    pub fn primary(mut self, value: GetPracticeResponseContactsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPracticeResponseContacts`].
    pub fn build(self) -> Result<GetPracticeResponseContacts, BuildError> {
        Ok(GetPracticeResponseContacts {
            compliance: self.compliance,
            primary: self.primary,
        })
    }
}

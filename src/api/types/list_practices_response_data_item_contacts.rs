pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPracticesResponseDataItemContacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<ListPracticesResponseDataItemContactsCompliance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<ListPracticesResponseDataItemContactsPrimary>,
}

impl ListPracticesResponseDataItemContacts {
    pub fn builder() -> ListPracticesResponseDataItemContactsBuilder {
        <ListPracticesResponseDataItemContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPracticesResponseDataItemContactsBuilder {
    compliance: Option<ListPracticesResponseDataItemContactsCompliance>,
    primary: Option<ListPracticesResponseDataItemContactsPrimary>,
}

impl ListPracticesResponseDataItemContactsBuilder {
    pub fn compliance(mut self, value: ListPracticesResponseDataItemContactsCompliance) -> Self {
        self.compliance = Some(value);
        self
    }

    pub fn primary(mut self, value: ListPracticesResponseDataItemContactsPrimary) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPracticesResponseDataItemContacts`].
    pub fn build(self) -> Result<ListPracticesResponseDataItemContacts, BuildError> {
        Ok(ListPracticesResponseDataItemContacts {
            compliance: self.compliance,
            primary: self.primary,
        })
    }
}

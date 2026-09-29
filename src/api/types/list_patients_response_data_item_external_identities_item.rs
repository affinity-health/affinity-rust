pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPatientsResponseDataItemExternalIdentitiesItem {
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub value: String,
}

impl ListPatientsResponseDataItemExternalIdentitiesItem {
    pub fn builder() -> ListPatientsResponseDataItemExternalIdentitiesItemBuilder {
        <ListPatientsResponseDataItemExternalIdentitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemExternalIdentitiesItemBuilder {
    source: Option<String>,
    value: Option<String>,
}

impl ListPatientsResponseDataItemExternalIdentitiesItemBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemExternalIdentitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](ListPatientsResponseDataItemExternalIdentitiesItemBuilder::source)
    /// - [`value`](ListPatientsResponseDataItemExternalIdentitiesItemBuilder::value)
    pub fn build(self) -> Result<ListPatientsResponseDataItemExternalIdentitiesItem, BuildError> {
        Ok(ListPatientsResponseDataItemExternalIdentitiesItem {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}

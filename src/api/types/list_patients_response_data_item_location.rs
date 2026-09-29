pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPatientsResponseDataItemLocation {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub status: ListPatientsResponseDataItemLocationStatus,
}

impl ListPatientsResponseDataItemLocation {
    pub fn builder() -> ListPatientsResponseDataItemLocationBuilder {
        <ListPatientsResponseDataItemLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemLocationBuilder {
    id: Option<String>,
    name: Option<String>,
    state: Option<String>,
    status: Option<ListPatientsResponseDataItemLocationStatus>,
}

impl ListPatientsResponseDataItemLocationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPatientsResponseDataItemLocationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListPatientsResponseDataItemLocationBuilder::id)
    /// - [`name`](ListPatientsResponseDataItemLocationBuilder::name)
    /// - [`status`](ListPatientsResponseDataItemLocationBuilder::status)
    pub fn build(self) -> Result<ListPatientsResponseDataItemLocation, BuildError> {
        Ok(ListPatientsResponseDataItemLocation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

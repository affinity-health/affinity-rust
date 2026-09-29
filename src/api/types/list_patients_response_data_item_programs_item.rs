pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPatientsResponseDataItemProgramsItem {
    #[serde(rename = "endedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    pub started_at: String,
    pub status: ListPatientsResponseDataItemProgramsItemStatus,
}

impl ListPatientsResponseDataItemProgramsItem {
    pub fn builder() -> ListPatientsResponseDataItemProgramsItemBuilder {
        <ListPatientsResponseDataItemProgramsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemProgramsItemBuilder {
    ended_at: Option<String>,
    name: Option<String>,
    started_at: Option<String>,
    status: Option<ListPatientsResponseDataItemProgramsItemStatus>,
}

impl ListPatientsResponseDataItemProgramsItemBuilder {
    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPatientsResponseDataItemProgramsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItemProgramsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ListPatientsResponseDataItemProgramsItemBuilder::name)
    /// - [`started_at`](ListPatientsResponseDataItemProgramsItemBuilder::started_at)
    /// - [`status`](ListPatientsResponseDataItemProgramsItemBuilder::status)
    pub fn build(self) -> Result<ListPatientsResponseDataItemProgramsItem, BuildError> {
        Ok(ListPatientsResponseDataItemProgramsItem {
            ended_at: self.ended_at,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            started_at: self
                .started_at
                .ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

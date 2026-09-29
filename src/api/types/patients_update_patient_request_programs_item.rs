pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePatientRequestProgramsItem {
    #[serde(rename = "endedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    pub started_at: String,
    pub status: UpdatePatientRequestProgramsItemStatus,
}

impl UpdatePatientRequestProgramsItem {
    pub fn builder() -> UpdatePatientRequestProgramsItemBuilder {
        <UpdatePatientRequestProgramsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestProgramsItemBuilder {
    ended_at: Option<String>,
    name: Option<String>,
    started_at: Option<String>,
    status: Option<UpdatePatientRequestProgramsItemStatus>,
}

impl UpdatePatientRequestProgramsItemBuilder {
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

    pub fn status(mut self, value: UpdatePatientRequestProgramsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientRequestProgramsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdatePatientRequestProgramsItemBuilder::name)
    /// - [`started_at`](UpdatePatientRequestProgramsItemBuilder::started_at)
    /// - [`status`](UpdatePatientRequestProgramsItemBuilder::status)
    pub fn build(self) -> Result<UpdatePatientRequestProgramsItem, BuildError> {
        Ok(UpdatePatientRequestProgramsItem {
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

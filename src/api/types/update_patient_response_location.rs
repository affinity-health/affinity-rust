pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdatePatientResponseLocation {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub status: UpdatePatientResponseLocationStatus,
}

impl UpdatePatientResponseLocation {
    pub fn builder() -> UpdatePatientResponseLocationBuilder {
        <UpdatePatientResponseLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseLocationBuilder {
    id: Option<String>,
    name: Option<String>,
    state: Option<String>,
    status: Option<UpdatePatientResponseLocationStatus>,
}

impl UpdatePatientResponseLocationBuilder {
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

    pub fn status(mut self, value: UpdatePatientResponseLocationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponseLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdatePatientResponseLocationBuilder::id)
    /// - [`name`](UpdatePatientResponseLocationBuilder::name)
    /// - [`status`](UpdatePatientResponseLocationBuilder::status)
    pub fn build(self) -> Result<UpdatePatientResponseLocation, BuildError> {
        Ok(UpdatePatientResponseLocation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

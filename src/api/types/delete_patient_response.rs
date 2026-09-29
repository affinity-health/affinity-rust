pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeletePatientResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
    pub object: DeletePatientResponseObject,
}

impl DeletePatientResponse {
    pub fn builder() -> DeletePatientResponseBuilder {
        <DeletePatientResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePatientResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
    object: Option<DeletePatientResponseObject>,
}

impl DeletePatientResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: DeletePatientResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeletePatientResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeletePatientResponseBuilder::deleted)
    /// - [`id`](DeletePatientResponseBuilder::id)
    /// - [`object`](DeletePatientResponseBuilder::object)
    pub fn build(self) -> Result<DeletePatientResponse, BuildError> {
        Ok(DeletePatientResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}

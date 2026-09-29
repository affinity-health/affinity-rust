pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
    pub fn builder() -> CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
        <CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder::id)
    /// - [`name`](CancelOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder::name)
    pub fn build(
        self,
    ) -> Result<CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo, BuildError> {
        Ok(
            CancelOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            },
        )
    }
}

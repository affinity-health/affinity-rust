pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl GetOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
    pub fn builder() -> GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
        <GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponseFulfillmentsItemExceptionsItemAssignedTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder::id)
    /// - [`name`](GetOrderResponseFulfillmentsItemExceptionsItemAssignedToBuilder::name)
    pub fn build(
        self,
    ) -> Result<GetOrderResponseFulfillmentsItemExceptionsItemAssignedTo, BuildError> {
        Ok(GetOrderResponseFulfillmentsItemExceptionsItemAssignedTo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

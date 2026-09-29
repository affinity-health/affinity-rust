pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo {
    pub fn builder() -> ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder {
        <ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder::id)
    /// - [`name`](ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedToBuilder::name)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo, BuildError>
    {
        Ok(
            ListOrdersResponseDataItemFulfillmentsItemExceptionsItemAssignedTo {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            },
        )
    }
}

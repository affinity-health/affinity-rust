pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemMetadata {}

impl ListOrdersResponseDataItemMetadata {
    pub fn builder() -> ListOrdersResponseDataItemMetadataBuilder {
        <ListOrdersResponseDataItemMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemMetadataBuilder {}

impl ListOrdersResponseDataItemMetadataBuilder {
    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemMetadata`].
    pub fn build(self) -> Result<ListOrdersResponseDataItemMetadata, BuildError> {
        Ok(ListOrdersResponseDataItemMetadata {})
    }
}

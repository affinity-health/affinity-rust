pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchResponseOrdersItemMetadata {}

impl CreateOrderBatchResponseOrdersItemMetadata {
    pub fn builder() -> CreateOrderBatchResponseOrdersItemMetadataBuilder {
        <CreateOrderBatchResponseOrdersItemMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchResponseOrdersItemMetadataBuilder {}

impl CreateOrderBatchResponseOrdersItemMetadataBuilder {
    /// Consumes the builder and constructs a [`CreateOrderBatchResponseOrdersItemMetadata`].
    pub fn build(self) -> Result<CreateOrderBatchResponseOrdersItemMetadata, BuildError> {
        Ok(CreateOrderBatchResponseOrdersItemMetadata {})
    }
}

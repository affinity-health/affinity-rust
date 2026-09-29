pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderResponseMetadata {}

impl CreateOrderResponseMetadata {
    pub fn builder() -> CreateOrderResponseMetadataBuilder {
        <CreateOrderResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderResponseMetadataBuilder {}

impl CreateOrderResponseMetadataBuilder {
    /// Consumes the builder and constructs a [`CreateOrderResponseMetadata`].
    pub fn build(self) -> Result<CreateOrderResponseMetadata, BuildError> {
        Ok(CreateOrderResponseMetadata {})
    }
}

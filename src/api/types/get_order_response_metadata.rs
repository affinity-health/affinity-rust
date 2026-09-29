pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponseMetadata {}

impl GetOrderResponseMetadata {
    pub fn builder() -> GetOrderResponseMetadataBuilder {
        <GetOrderResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseMetadataBuilder {}

impl GetOrderResponseMetadataBuilder {
    /// Consumes the builder and constructs a [`GetOrderResponseMetadata`].
    pub fn build(self) -> Result<GetOrderResponseMetadata, BuildError> {
        Ok(GetOrderResponseMetadata {})
    }
}

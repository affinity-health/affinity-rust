pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseMetadata {}

impl CancelOrderResponseMetadata {
    pub fn builder() -> CancelOrderResponseMetadataBuilder {
        <CancelOrderResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseMetadataBuilder {}

impl CancelOrderResponseMetadataBuilder {
    /// Consumes the builder and constructs a [`CancelOrderResponseMetadata`].
    pub fn build(self) -> Result<CancelOrderResponseMetadata, BuildError> {
        Ok(CancelOrderResponseMetadata {})
    }
}

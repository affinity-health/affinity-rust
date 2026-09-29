pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderPrescriptionResponseMetadata {}

impl UpdateOrderPrescriptionResponseMetadata {
    pub fn builder() -> UpdateOrderPrescriptionResponseMetadataBuilder {
        <UpdateOrderPrescriptionResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionResponseMetadataBuilder {}

impl UpdateOrderPrescriptionResponseMetadataBuilder {
    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionResponseMetadata`].
    pub fn build(self) -> Result<UpdateOrderPrescriptionResponseMetadata, BuildError> {
        Ok(UpdateOrderPrescriptionResponseMetadata {})
    }
}

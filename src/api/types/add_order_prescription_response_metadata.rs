pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddOrderPrescriptionResponseMetadata {}

impl AddOrderPrescriptionResponseMetadata {
    pub fn builder() -> AddOrderPrescriptionResponseMetadataBuilder {
        <AddOrderPrescriptionResponseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionResponseMetadataBuilder {}

impl AddOrderPrescriptionResponseMetadataBuilder {
    /// Consumes the builder and constructs a [`AddOrderPrescriptionResponseMetadata`].
    pub fn build(self) -> Result<AddOrderPrescriptionResponseMetadata, BuildError> {
        Ok(AddOrderPrescriptionResponseMetadata {})
    }
}

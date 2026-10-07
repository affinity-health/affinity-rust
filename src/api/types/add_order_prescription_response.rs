pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddOrderPrescriptionResponse {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    pub object: AddOrderPrescriptionResponseObject,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, Option<AddOrderPrescriptionResponseMetadataValue>>,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub prescriptions: Vec<AddOrderPrescriptionResponsePrescriptionsItem>,
}

impl AddOrderPrescriptionResponse {
    pub fn builder() -> AddOrderPrescriptionResponseBuilder {
        <AddOrderPrescriptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionResponseBuilder {
    revision: Option<String>,
    object: Option<AddOrderPrescriptionResponseObject>,
    external_order_id: Option<String>,
    metadata: Option<HashMap<String, Option<AddOrderPrescriptionResponseMetadataValue>>>,
    order_id: Option<String>,
    prescription_id: Option<String>,
    prescriptions: Option<Vec<AddOrderPrescriptionResponsePrescriptionsItem>>,
}

impl AddOrderPrescriptionResponseBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn object(mut self, value: AddOrderPrescriptionResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<AddOrderPrescriptionResponseMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn prescriptions(
        mut self,
        value: Vec<AddOrderPrescriptionResponsePrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](AddOrderPrescriptionResponseBuilder::revision)
    /// - [`object`](AddOrderPrescriptionResponseBuilder::object)
    /// - [`metadata`](AddOrderPrescriptionResponseBuilder::metadata)
    /// - [`order_id`](AddOrderPrescriptionResponseBuilder::order_id)
    /// - [`prescription_id`](AddOrderPrescriptionResponseBuilder::prescription_id)
    /// - [`prescriptions`](AddOrderPrescriptionResponseBuilder::prescriptions)
    pub fn build(self) -> Result<AddOrderPrescriptionResponse, BuildError> {
        Ok(AddOrderPrescriptionResponse {
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            external_order_id: self.external_order_id,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
        })
    }
}

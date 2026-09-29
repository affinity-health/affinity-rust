pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderResponse {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<CreateOrderResponseOtcItemsItem>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: CreateOrderResponseMetadata,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub object: CreateOrderResponseObject,
    #[serde(rename = "patientId")]
    #[serde(default)]
    pub patient_id: String,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(default)]
    pub prescriptions: Vec<CreateOrderResponsePrescriptionsItem>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: CreateOrderResponseStatus,
}

impl CreateOrderResponse {
    pub fn builder() -> CreateOrderResponseBuilder {
        <CreateOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderResponseBuilder {
    revision: Option<String>,
    otc_items: Option<Vec<CreateOrderResponseOtcItemsItem>>,
    external_order_id: Option<String>,
    metadata: Option<CreateOrderResponseMetadata>,
    created_at: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<CreateOrderResponseObject>,
    patient_id: Option<String>,
    practice_id: Option<String>,
    prescriptions: Option<Vec<CreateOrderResponsePrescriptionsItem>>,
    user_id: Option<String>,
    status: Option<CreateOrderResponseStatus>,
}

impl CreateOrderResponseBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn otc_items(mut self, value: Vec<CreateOrderResponseOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: CreateOrderResponseMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: CreateOrderResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn prescriptions(mut self, value: Vec<CreateOrderResponsePrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](CreateOrderResponseBuilder::revision)
    /// - [`otc_items`](CreateOrderResponseBuilder::otc_items)
    /// - [`metadata`](CreateOrderResponseBuilder::metadata)
    /// - [`created_at`](CreateOrderResponseBuilder::created_at)
    /// - [`id`](CreateOrderResponseBuilder::id)
    /// - [`livemode`](CreateOrderResponseBuilder::livemode)
    /// - [`object`](CreateOrderResponseBuilder::object)
    /// - [`patient_id`](CreateOrderResponseBuilder::patient_id)
    /// - [`practice_id`](CreateOrderResponseBuilder::practice_id)
    /// - [`prescriptions`](CreateOrderResponseBuilder::prescriptions)
    /// - [`status`](CreateOrderResponseBuilder::status)
    pub fn build(self) -> Result<CreateOrderResponse, BuildError> {
        Ok(CreateOrderResponse {
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            otc_items: self
                .otc_items
                .ok_or_else(|| BuildError::missing_field("otc_items"))?,
            external_order_id: self.external_order_id,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            patient_id: self
                .patient_id
                .ok_or_else(|| BuildError::missing_field("patient_id"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            user_id: self.user_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

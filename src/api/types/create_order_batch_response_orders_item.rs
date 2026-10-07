pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderBatchResponseOrdersItem {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<CreateOrderBatchResponseOrdersItemOtcItemsItem>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, Option<CreateOrderBatchResponseOrdersItemMetadataValue>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub object: CreateOrderBatchResponseOrdersItemObject,
    #[serde(rename = "patientId")]
    #[serde(default)]
    pub patient_id: String,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(default)]
    pub prescriptions: Vec<CreateOrderBatchResponseOrdersItemPrescriptionsItem>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub status: CreateOrderBatchResponseOrdersItemStatus,
}

impl CreateOrderBatchResponseOrdersItem {
    pub fn builder() -> CreateOrderBatchResponseOrdersItemBuilder {
        <CreateOrderBatchResponseOrdersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchResponseOrdersItemBuilder {
    revision: Option<String>,
    otc_items: Option<Vec<CreateOrderBatchResponseOrdersItemOtcItemsItem>>,
    external_order_id: Option<String>,
    metadata: Option<HashMap<String, Option<CreateOrderBatchResponseOrdersItemMetadataValue>>>,
    created_at: Option<String>,
    id: Option<String>,
    livemode: Option<bool>,
    object: Option<CreateOrderBatchResponseOrdersItemObject>,
    patient_id: Option<String>,
    practice_id: Option<String>,
    prescriptions: Option<Vec<CreateOrderBatchResponseOrdersItemPrescriptionsItem>>,
    user_id: Option<String>,
    status: Option<CreateOrderBatchResponseOrdersItemStatus>,
}

impl CreateOrderBatchResponseOrdersItemBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn otc_items(mut self, value: Vec<CreateOrderBatchResponseOrdersItemOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<CreateOrderBatchResponseOrdersItemMetadataValue>>,
    ) -> Self {
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

    pub fn object(mut self, value: CreateOrderBatchResponseOrdersItemObject) -> Self {
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

    pub fn prescriptions(
        mut self,
        value: Vec<CreateOrderBatchResponseOrdersItemPrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateOrderBatchResponseOrdersItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchResponseOrdersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](CreateOrderBatchResponseOrdersItemBuilder::revision)
    /// - [`otc_items`](CreateOrderBatchResponseOrdersItemBuilder::otc_items)
    /// - [`metadata`](CreateOrderBatchResponseOrdersItemBuilder::metadata)
    /// - [`created_at`](CreateOrderBatchResponseOrdersItemBuilder::created_at)
    /// - [`id`](CreateOrderBatchResponseOrdersItemBuilder::id)
    /// - [`livemode`](CreateOrderBatchResponseOrdersItemBuilder::livemode)
    /// - [`object`](CreateOrderBatchResponseOrdersItemBuilder::object)
    /// - [`patient_id`](CreateOrderBatchResponseOrdersItemBuilder::patient_id)
    /// - [`practice_id`](CreateOrderBatchResponseOrdersItemBuilder::practice_id)
    /// - [`prescriptions`](CreateOrderBatchResponseOrdersItemBuilder::prescriptions)
    /// - [`status`](CreateOrderBatchResponseOrdersItemBuilder::status)
    pub fn build(self) -> Result<CreateOrderBatchResponseOrdersItem, BuildError> {
        Ok(CreateOrderBatchResponseOrdersItem {
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

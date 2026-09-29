pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListOrdersResponseDataItem {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<ListOrdersResponseDataItemOtcItemsItem>,
    /// Snapshot of the practice-facing medication total. Null until every prescription has recorded submission pricing. Excludes shipping and supplies.
    #[serde(rename = "practiceMedicationTotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_medication_total_cents: Option<i64>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: ListOrdersResponseDataItemMetadata,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub fulfillments: Vec<ListOrdersResponseDataItemFulfillmentsItem>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    pub lifecycle_events: Vec<ListOrdersResponseDataItemLifecycleEventsItem>,
    #[serde(default)]
    pub livemode: bool,
    pub object: ListOrdersResponseDataItemObject,
    #[serde(rename = "patientExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_external_id: Option<String>,
    #[serde(rename = "patientId")]
    #[serde(default)]
    pub patient_id: String,
    #[serde(rename = "patientName")]
    #[serde(default)]
    pub patient_name: String,
    /// The patient's current clinical state. This is not the saved delivery state; use each prescription's deliveryAddress for shipping.
    #[serde(rename = "patientState")]
    #[serde(default)]
    pub patient_state: String,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "prescriberName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_name: Option<String>,
    #[serde(rename = "prescriberNpi")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber_npi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<ListOrdersResponseDataItemReview>,
    #[serde(default)]
    pub prescriptions: Vec<ListOrdersResponseDataItemPrescriptionsItem>,
    pub status: ListOrdersResponseDataItemStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl ListOrdersResponseDataItem {
    pub fn builder() -> ListOrdersResponseDataItemBuilder {
        <ListOrdersResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemBuilder {
    revision: Option<String>,
    otc_items: Option<Vec<ListOrdersResponseDataItemOtcItemsItem>>,
    practice_medication_total_cents: Option<i64>,
    external_order_id: Option<String>,
    metadata: Option<ListOrdersResponseDataItemMetadata>,
    created_at: Option<String>,
    fulfillments: Option<Vec<ListOrdersResponseDataItemFulfillmentsItem>>,
    id: Option<String>,
    lifecycle_events: Option<Vec<ListOrdersResponseDataItemLifecycleEventsItem>>,
    livemode: Option<bool>,
    object: Option<ListOrdersResponseDataItemObject>,
    patient_external_id: Option<String>,
    patient_id: Option<String>,
    patient_name: Option<String>,
    patient_state: Option<String>,
    practice_id: Option<String>,
    prescriber_name: Option<String>,
    prescriber_npi: Option<String>,
    review: Option<ListOrdersResponseDataItemReview>,
    prescriptions: Option<Vec<ListOrdersResponseDataItemPrescriptionsItem>>,
    status: Option<ListOrdersResponseDataItemStatus>,
    updated_at: Option<String>,
}

impl ListOrdersResponseDataItemBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn otc_items(mut self, value: Vec<ListOrdersResponseDataItemOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn practice_medication_total_cents(mut self, value: i64) -> Self {
        self.practice_medication_total_cents = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: ListOrdersResponseDataItemMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn fulfillments(mut self, value: Vec<ListOrdersResponseDataItemFulfillmentsItem>) -> Self {
        self.fulfillments = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lifecycle_events(
        mut self,
        value: Vec<ListOrdersResponseDataItemLifecycleEventsItem>,
    ) -> Self {
        self.lifecycle_events = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: ListOrdersResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn patient_external_id(mut self, value: impl Into<String>) -> Self {
        self.patient_external_id = Some(value.into());
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient_name(mut self, value: impl Into<String>) -> Self {
        self.patient_name = Some(value.into());
        self
    }

    pub fn patient_state(mut self, value: impl Into<String>) -> Self {
        self.patient_state = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn prescriber_name(mut self, value: impl Into<String>) -> Self {
        self.prescriber_name = Some(value.into());
        self
    }

    pub fn prescriber_npi(mut self, value: impl Into<String>) -> Self {
        self.prescriber_npi = Some(value.into());
        self
    }

    pub fn review(mut self, value: ListOrdersResponseDataItemReview) -> Self {
        self.review = Some(value);
        self
    }

    pub fn prescriptions(
        mut self,
        value: Vec<ListOrdersResponseDataItemPrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn status(mut self, value: ListOrdersResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](ListOrdersResponseDataItemBuilder::revision)
    /// - [`otc_items`](ListOrdersResponseDataItemBuilder::otc_items)
    /// - [`metadata`](ListOrdersResponseDataItemBuilder::metadata)
    /// - [`created_at`](ListOrdersResponseDataItemBuilder::created_at)
    /// - [`fulfillments`](ListOrdersResponseDataItemBuilder::fulfillments)
    /// - [`id`](ListOrdersResponseDataItemBuilder::id)
    /// - [`lifecycle_events`](ListOrdersResponseDataItemBuilder::lifecycle_events)
    /// - [`livemode`](ListOrdersResponseDataItemBuilder::livemode)
    /// - [`object`](ListOrdersResponseDataItemBuilder::object)
    /// - [`patient_id`](ListOrdersResponseDataItemBuilder::patient_id)
    /// - [`patient_name`](ListOrdersResponseDataItemBuilder::patient_name)
    /// - [`patient_state`](ListOrdersResponseDataItemBuilder::patient_state)
    /// - [`practice_id`](ListOrdersResponseDataItemBuilder::practice_id)
    /// - [`prescriptions`](ListOrdersResponseDataItemBuilder::prescriptions)
    /// - [`status`](ListOrdersResponseDataItemBuilder::status)
    /// - [`updated_at`](ListOrdersResponseDataItemBuilder::updated_at)
    pub fn build(self) -> Result<ListOrdersResponseDataItem, BuildError> {
        Ok(ListOrdersResponseDataItem {
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            otc_items: self
                .otc_items
                .ok_or_else(|| BuildError::missing_field("otc_items"))?,
            practice_medication_total_cents: self.practice_medication_total_cents,
            external_order_id: self.external_order_id,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            fulfillments: self
                .fulfillments
                .ok_or_else(|| BuildError::missing_field("fulfillments"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lifecycle_events: self
                .lifecycle_events
                .ok_or_else(|| BuildError::missing_field("lifecycle_events"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            patient_external_id: self.patient_external_id,
            patient_id: self
                .patient_id
                .ok_or_else(|| BuildError::missing_field("patient_id"))?,
            patient_name: self
                .patient_name
                .ok_or_else(|| BuildError::missing_field("patient_name"))?,
            patient_state: self
                .patient_state
                .ok_or_else(|| BuildError::missing_field("patient_state"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            prescriber_name: self.prescriber_name,
            prescriber_npi: self.prescriber_npi,
            review: self.review,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}

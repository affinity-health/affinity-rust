pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetOrderResponse {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<GetOrderResponseOtcItemsItem>,
    /// Snapshot of the practice-facing medication total. Null until every prescription has recorded submission pricing. Excludes shipping and supplies.
    #[serde(rename = "practiceMedicationTotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_medication_total_cents: Option<i64>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: GetOrderResponseMetadata,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub fulfillments: Vec<GetOrderResponseFulfillmentsItem>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    pub lifecycle_events: Vec<GetOrderResponseLifecycleEventsItem>,
    #[serde(default)]
    pub livemode: bool,
    pub object: GetOrderResponseObject,
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
    pub review: Option<GetOrderResponseReview>,
    #[serde(default)]
    pub prescriptions: Vec<GetOrderResponsePrescriptionsItem>,
    pub status: GetOrderResponseStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl GetOrderResponse {
    pub fn builder() -> GetOrderResponseBuilder {
        <GetOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponseBuilder {
    revision: Option<String>,
    otc_items: Option<Vec<GetOrderResponseOtcItemsItem>>,
    practice_medication_total_cents: Option<i64>,
    external_order_id: Option<String>,
    metadata: Option<GetOrderResponseMetadata>,
    created_at: Option<String>,
    fulfillments: Option<Vec<GetOrderResponseFulfillmentsItem>>,
    id: Option<String>,
    lifecycle_events: Option<Vec<GetOrderResponseLifecycleEventsItem>>,
    livemode: Option<bool>,
    object: Option<GetOrderResponseObject>,
    patient_external_id: Option<String>,
    patient_id: Option<String>,
    patient_name: Option<String>,
    patient_state: Option<String>,
    practice_id: Option<String>,
    prescriber_name: Option<String>,
    prescriber_npi: Option<String>,
    review: Option<GetOrderResponseReview>,
    prescriptions: Option<Vec<GetOrderResponsePrescriptionsItem>>,
    status: Option<GetOrderResponseStatus>,
    updated_at: Option<String>,
}

impl GetOrderResponseBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn otc_items(mut self, value: Vec<GetOrderResponseOtcItemsItem>) -> Self {
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

    pub fn metadata(mut self, value: GetOrderResponseMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn fulfillments(mut self, value: Vec<GetOrderResponseFulfillmentsItem>) -> Self {
        self.fulfillments = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lifecycle_events(mut self, value: Vec<GetOrderResponseLifecycleEventsItem>) -> Self {
        self.lifecycle_events = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: GetOrderResponseObject) -> Self {
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

    pub fn review(mut self, value: GetOrderResponseReview) -> Self {
        self.review = Some(value);
        self
    }

    pub fn prescriptions(mut self, value: Vec<GetOrderResponsePrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn status(mut self, value: GetOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](GetOrderResponseBuilder::revision)
    /// - [`otc_items`](GetOrderResponseBuilder::otc_items)
    /// - [`metadata`](GetOrderResponseBuilder::metadata)
    /// - [`created_at`](GetOrderResponseBuilder::created_at)
    /// - [`fulfillments`](GetOrderResponseBuilder::fulfillments)
    /// - [`id`](GetOrderResponseBuilder::id)
    /// - [`lifecycle_events`](GetOrderResponseBuilder::lifecycle_events)
    /// - [`livemode`](GetOrderResponseBuilder::livemode)
    /// - [`object`](GetOrderResponseBuilder::object)
    /// - [`patient_id`](GetOrderResponseBuilder::patient_id)
    /// - [`patient_name`](GetOrderResponseBuilder::patient_name)
    /// - [`patient_state`](GetOrderResponseBuilder::patient_state)
    /// - [`practice_id`](GetOrderResponseBuilder::practice_id)
    /// - [`prescriptions`](GetOrderResponseBuilder::prescriptions)
    /// - [`status`](GetOrderResponseBuilder::status)
    /// - [`updated_at`](GetOrderResponseBuilder::updated_at)
    pub fn build(self) -> Result<GetOrderResponse, BuildError> {
        Ok(GetOrderResponse {
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

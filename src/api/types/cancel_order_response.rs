pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelOrderResponse {
    /// Opaque revision of the complete order prescription set. Send the revision you reviewed as expectedRevision; never replace it automatically after a conflict.
    #[serde(default)]
    pub revision: String,
    #[serde(rename = "otcItems")]
    #[serde(default)]
    pub otc_items: Vec<CancelOrderResponseOtcItemsItem>,
    /// Snapshot of the practice-facing medication total. Null until every prescription has recorded submission pricing. Excludes shipping and supplies.
    #[serde(rename = "practiceMedicationTotalCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_medication_total_cents: Option<i64>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, Option<CancelOrderResponseMetadataValue>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub fulfillments: Vec<CancelOrderResponseFulfillmentsItem>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    pub lifecycle_events: Vec<CancelOrderResponseLifecycleEventsItem>,
    #[serde(default)]
    pub livemode: bool,
    pub object: CancelOrderResponseObject,
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
    pub review: Option<CancelOrderResponseReview>,
    #[serde(default)]
    pub prescriptions: Vec<CancelOrderResponsePrescriptionsItem>,
    pub status: CancelOrderResponseStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    pub cancellation: CancelOrderResponseCancellation,
}

impl CancelOrderResponse {
    pub fn builder() -> CancelOrderResponseBuilder {
        <CancelOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseBuilder {
    revision: Option<String>,
    otc_items: Option<Vec<CancelOrderResponseOtcItemsItem>>,
    practice_medication_total_cents: Option<i64>,
    external_order_id: Option<String>,
    metadata: Option<HashMap<String, Option<CancelOrderResponseMetadataValue>>>,
    created_at: Option<String>,
    fulfillments: Option<Vec<CancelOrderResponseFulfillmentsItem>>,
    id: Option<String>,
    lifecycle_events: Option<Vec<CancelOrderResponseLifecycleEventsItem>>,
    livemode: Option<bool>,
    object: Option<CancelOrderResponseObject>,
    patient_external_id: Option<String>,
    patient_id: Option<String>,
    patient_name: Option<String>,
    patient_state: Option<String>,
    practice_id: Option<String>,
    prescriber_name: Option<String>,
    prescriber_npi: Option<String>,
    review: Option<CancelOrderResponseReview>,
    prescriptions: Option<Vec<CancelOrderResponsePrescriptionsItem>>,
    status: Option<CancelOrderResponseStatus>,
    updated_at: Option<String>,
    cancellation: Option<CancelOrderResponseCancellation>,
}

impl CancelOrderResponseBuilder {
    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn otc_items(mut self, value: Vec<CancelOrderResponseOtcItemsItem>) -> Self {
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

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<CancelOrderResponseMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn fulfillments(mut self, value: Vec<CancelOrderResponseFulfillmentsItem>) -> Self {
        self.fulfillments = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lifecycle_events(mut self, value: Vec<CancelOrderResponseLifecycleEventsItem>) -> Self {
        self.lifecycle_events = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn object(mut self, value: CancelOrderResponseObject) -> Self {
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

    pub fn review(mut self, value: CancelOrderResponseReview) -> Self {
        self.review = Some(value);
        self
    }

    pub fn prescriptions(mut self, value: Vec<CancelOrderResponsePrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn status(mut self, value: CancelOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn cancellation(mut self, value: CancelOrderResponseCancellation) -> Self {
        self.cancellation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](CancelOrderResponseBuilder::revision)
    /// - [`otc_items`](CancelOrderResponseBuilder::otc_items)
    /// - [`metadata`](CancelOrderResponseBuilder::metadata)
    /// - [`created_at`](CancelOrderResponseBuilder::created_at)
    /// - [`fulfillments`](CancelOrderResponseBuilder::fulfillments)
    /// - [`id`](CancelOrderResponseBuilder::id)
    /// - [`lifecycle_events`](CancelOrderResponseBuilder::lifecycle_events)
    /// - [`livemode`](CancelOrderResponseBuilder::livemode)
    /// - [`object`](CancelOrderResponseBuilder::object)
    /// - [`patient_id`](CancelOrderResponseBuilder::patient_id)
    /// - [`patient_name`](CancelOrderResponseBuilder::patient_name)
    /// - [`patient_state`](CancelOrderResponseBuilder::patient_state)
    /// - [`practice_id`](CancelOrderResponseBuilder::practice_id)
    /// - [`prescriptions`](CancelOrderResponseBuilder::prescriptions)
    /// - [`status`](CancelOrderResponseBuilder::status)
    /// - [`updated_at`](CancelOrderResponseBuilder::updated_at)
    /// - [`cancellation`](CancelOrderResponseBuilder::cancellation)
    pub fn build(self) -> Result<CancelOrderResponse, BuildError> {
        Ok(CancelOrderResponse {
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
            cancellation: self
                .cancellation
                .ok_or_else(|| BuildError::missing_field("cancellation"))?,
        })
    }
}

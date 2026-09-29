pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<CreateOrderRequestPrescriber>,
    #[serde(rename = "otcItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otc_items: Option<Vec<CreateOrderRequestOtcItemsItem>>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<CreateOrderRequestMetadataValue>>>,
    #[serde(rename = "patientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<CreateOrderRequestPatient>,
    #[serde(rename = "shippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_id: Option<String>,
    #[serde(default)]
    pub prescriptions: Vec<CreateOrderRequestPrescriptionsItem>,
}

impl CreateOrderRequest {
    pub fn builder() -> CreateOrderRequestBuilder {
        <CreateOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestBuilder {
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<CreateOrderRequestPrescriber>,
    otc_items: Option<Vec<CreateOrderRequestOtcItemsItem>>,
    external_order_id: Option<String>,
    metadata: Option<HashMap<String, Option<CreateOrderRequestMetadataValue>>>,
    patient_id: Option<String>,
    patient: Option<CreateOrderRequestPatient>,
    shipping_address_id: Option<String>,
    prescriptions: Option<Vec<CreateOrderRequestPrescriptionsItem>>,
}

impl CreateOrderRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: CreateOrderRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    pub fn otc_items(mut self, value: Vec<CreateOrderRequestOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<CreateOrderRequestMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient(mut self, value: CreateOrderRequestPatient) -> Self {
        self.patient = Some(value);
        self
    }

    pub fn shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.shipping_address_id = Some(value.into());
        self
    }

    pub fn prescriptions(mut self, value: Vec<CreateOrderRequestPrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](CreateOrderRequestBuilder::practice_id)
    /// - [`prescriptions`](CreateOrderRequestBuilder::prescriptions)
    pub fn build(self) -> Result<CreateOrderRequest, BuildError> {
        Ok(CreateOrderRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
            otc_items: self.otc_items,
            external_order_id: self.external_order_id,
            metadata: self.metadata,
            patient_id: self.patient_id,
            patient: self.patient,
            shipping_address_id: self.shipping_address_id,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
        })
    }
}

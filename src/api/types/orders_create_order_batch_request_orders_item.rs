pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderBatchRequestOrdersItem {
    #[serde(rename = "otcItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otc_items: Option<Vec<CreateOrderBatchRequestOrdersItemOtcItemsItem>>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<CreateOrderBatchRequestOrdersItemMetadataValue>>>,
    #[serde(rename = "patientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<CreateOrderBatchRequestOrdersItemPatient>,
    #[serde(rename = "shippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_id: Option<String>,
    #[serde(default)]
    pub prescriptions: Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItem>,
}

impl CreateOrderBatchRequestOrdersItem {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemBuilder {
        <CreateOrderBatchRequestOrdersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemBuilder {
    otc_items: Option<Vec<CreateOrderBatchRequestOrdersItemOtcItemsItem>>,
    external_order_id: Option<String>,
    metadata: Option<HashMap<String, Option<CreateOrderBatchRequestOrdersItemMetadataValue>>>,
    patient_id: Option<String>,
    patient: Option<CreateOrderBatchRequestOrdersItemPatient>,
    shipping_address_id: Option<String>,
    prescriptions: Option<Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItem>>,
}

impl CreateOrderBatchRequestOrdersItemBuilder {
    pub fn otc_items(mut self, value: Vec<CreateOrderBatchRequestOrdersItemOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<CreateOrderBatchRequestOrdersItemMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient(mut self, value: CreateOrderBatchRequestOrdersItemPatient) -> Self {
        self.patient = Some(value);
        self
    }

    pub fn shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.shipping_address_id = Some(value.into());
        self
    }

    pub fn prescriptions(
        mut self,
        value: Vec<CreateOrderBatchRequestOrdersItemPrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prescriptions`](CreateOrderBatchRequestOrdersItemBuilder::prescriptions)
    pub fn build(self) -> Result<CreateOrderBatchRequestOrdersItem, BuildError> {
        Ok(CreateOrderBatchRequestOrdersItem {
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

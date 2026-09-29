pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequest {
    #[serde(rename = "otcItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otc_items: Option<Vec<PreviewOrderRequestOtcItemsItem>>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "patientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    #[serde(rename = "patientExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<PreviewOrderRequestPatient>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<PreviewOrderRequestPrescriber>,
    #[serde(rename = "shippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_id: Option<String>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub prescriptions: Vec<PreviewOrderRequestPrescriptionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PreviewOrderRequestShipping>,
}

impl PreviewOrderRequest {
    pub fn builder() -> PreviewOrderRequestBuilder {
        <PreviewOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestBuilder {
    otc_items: Option<Vec<PreviewOrderRequestOtcItemsItem>>,
    practice_id: Option<String>,
    patient_id: Option<String>,
    patient_external_id: Option<String>,
    patient: Option<PreviewOrderRequestPatient>,
    user_id: Option<String>,
    prescriber: Option<PreviewOrderRequestPrescriber>,
    shipping_address_id: Option<String>,
    external_order_id: Option<String>,
    prescriptions: Option<Vec<PreviewOrderRequestPrescriptionsItem>>,
    shipping: Option<PreviewOrderRequestShipping>,
}

impl PreviewOrderRequestBuilder {
    pub fn otc_items(mut self, value: Vec<PreviewOrderRequestOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient_external_id(mut self, value: impl Into<String>) -> Self {
        self.patient_external_id = Some(value.into());
        self
    }

    pub fn patient(mut self, value: PreviewOrderRequestPatient) -> Self {
        self.patient = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: PreviewOrderRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    pub fn shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.shipping_address_id = Some(value.into());
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn prescriptions(mut self, value: Vec<PreviewOrderRequestPrescriptionsItem>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn shipping(mut self, value: PreviewOrderRequestShipping) -> Self {
        self.shipping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](PreviewOrderRequestBuilder::practice_id)
    /// - [`prescriptions`](PreviewOrderRequestBuilder::prescriptions)
    pub fn build(self) -> Result<PreviewOrderRequest, BuildError> {
        Ok(PreviewOrderRequest {
            otc_items: self.otc_items,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            patient_id: self.patient_id,
            patient_external_id: self.patient_external_id,
            patient: self.patient,
            user_id: self.user_id,
            prescriber: self.prescriber,
            shipping_address_id: self.shipping_address_id,
            external_order_id: self.external_order_id,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            shipping: self.shipping,
        })
    }
}

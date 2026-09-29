pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponseOrderInput {
    #[serde(rename = "otcItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otc_items: Option<Vec<PreviewOrderResponseOrderInputOtcItemsItem>>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<PreviewOrderResponseOrderInputPrescriber>,
    #[serde(rename = "shippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_id: Option<String>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(default)]
    pub prescriptions: Vec<PreviewOrderResponseOrderInputPrescriptionsItem>,
    #[serde(rename = "patientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient: Option<PreviewOrderResponseOrderInputPatient>,
}

impl PreviewOrderResponseOrderInput {
    pub fn builder() -> PreviewOrderResponseOrderInputBuilder {
        <PreviewOrderResponseOrderInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputBuilder {
    otc_items: Option<Vec<PreviewOrderResponseOrderInputOtcItemsItem>>,
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<PreviewOrderResponseOrderInputPrescriber>,
    shipping_address_id: Option<String>,
    external_order_id: Option<String>,
    prescriptions: Option<Vec<PreviewOrderResponseOrderInputPrescriptionsItem>>,
    patient_id: Option<String>,
    patient: Option<PreviewOrderResponseOrderInputPatient>,
}

impl PreviewOrderResponseOrderInputBuilder {
    pub fn otc_items(mut self, value: Vec<PreviewOrderResponseOrderInputOtcItemsItem>) -> Self {
        self.otc_items = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: PreviewOrderResponseOrderInputPrescriber) -> Self {
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

    pub fn prescriptions(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient(mut self, value: PreviewOrderResponseOrderInputPatient) -> Self {
        self.patient = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](PreviewOrderResponseOrderInputBuilder::practice_id)
    /// - [`prescriptions`](PreviewOrderResponseOrderInputBuilder::prescriptions)
    pub fn build(self) -> Result<PreviewOrderResponseOrderInput, BuildError> {
        Ok(PreviewOrderResponseOrderInput {
            otc_items: self.otc_items,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
            shipping_address_id: self.shipping_address_id,
            external_order_id: self.external_order_id,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            patient_id: self.patient_id,
            patient: self.patient,
        })
    }
}

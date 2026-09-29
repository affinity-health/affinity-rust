pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignAndSubmitOrderResponsePrescriptionsItem {
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    pub status: SignAndSubmitOrderResponsePrescriptionsItemStatus,
    #[serde(rename = "fulfillmentOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SignAndSubmitOrderResponsePrescriptionsItemError>,
}

impl SignAndSubmitOrderResponsePrescriptionsItem {
    pub fn builder() -> SignAndSubmitOrderResponsePrescriptionsItemBuilder {
        <SignAndSubmitOrderResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderResponsePrescriptionsItemBuilder {
    prescription_id: Option<String>,
    status: Option<SignAndSubmitOrderResponsePrescriptionsItemStatus>,
    fulfillment_order_id: Option<String>,
    error: Option<SignAndSubmitOrderResponsePrescriptionsItemError>,
}

impl SignAndSubmitOrderResponsePrescriptionsItemBuilder {
    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: SignAndSubmitOrderResponsePrescriptionsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn fulfillment_order_id(mut self, value: impl Into<String>) -> Self {
        self.fulfillment_order_id = Some(value.into());
        self
    }

    pub fn error(mut self, value: SignAndSubmitOrderResponsePrescriptionsItemError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prescription_id`](SignAndSubmitOrderResponsePrescriptionsItemBuilder::prescription_id)
    /// - [`status`](SignAndSubmitOrderResponsePrescriptionsItemBuilder::status)
    pub fn build(self) -> Result<SignAndSubmitOrderResponsePrescriptionsItem, BuildError> {
        Ok(SignAndSubmitOrderResponsePrescriptionsItem {
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            fulfillment_order_id: self.fulfillment_order_id,
            error: self.error,
        })
    }
}

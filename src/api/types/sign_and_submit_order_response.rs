pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignAndSubmitOrderResponse {
    pub object: SignAndSubmitOrderResponseObject,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(rename = "signedAt")]
    #[serde(default)]
    pub signed_at: String,
    pub status: SignAndSubmitOrderResponseStatus,
    #[serde(default)]
    pub prescriptions: Vec<SignAndSubmitOrderResponsePrescriptionsItem>,
}

impl SignAndSubmitOrderResponse {
    pub fn builder() -> SignAndSubmitOrderResponseBuilder {
        <SignAndSubmitOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderResponseBuilder {
    object: Option<SignAndSubmitOrderResponseObject>,
    order_id: Option<String>,
    signed_at: Option<String>,
    status: Option<SignAndSubmitOrderResponseStatus>,
    prescriptions: Option<Vec<SignAndSubmitOrderResponsePrescriptionsItem>>,
}

impl SignAndSubmitOrderResponseBuilder {
    pub fn object(mut self, value: SignAndSubmitOrderResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn signed_at(mut self, value: impl Into<String>) -> Self {
        self.signed_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: SignAndSubmitOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn prescriptions(
        mut self,
        value: Vec<SignAndSubmitOrderResponsePrescriptionsItem>,
    ) -> Self {
        self.prescriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](SignAndSubmitOrderResponseBuilder::object)
    /// - [`order_id`](SignAndSubmitOrderResponseBuilder::order_id)
    /// - [`signed_at`](SignAndSubmitOrderResponseBuilder::signed_at)
    /// - [`status`](SignAndSubmitOrderResponseBuilder::status)
    /// - [`prescriptions`](SignAndSubmitOrderResponseBuilder::prescriptions)
    pub fn build(self) -> Result<SignAndSubmitOrderResponse, BuildError> {
        Ok(SignAndSubmitOrderResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            signed_at: self
                .signed_at
                .ok_or_else(|| BuildError::missing_field("signed_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
        })
    }
}

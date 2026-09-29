pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SignOrderResponse {
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub prescriptions: Vec<String>,
    #[serde(rename = "signedAt")]
    #[serde(default)]
    pub signed_at: String,
    pub status: SignOrderResponseStatus,
}

impl SignOrderResponse {
    pub fn builder() -> SignOrderResponseBuilder {
        <SignOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignOrderResponseBuilder {
    order_id: Option<String>,
    prescriptions: Option<Vec<String>>,
    signed_at: Option<String>,
    status: Option<SignOrderResponseStatus>,
}

impl SignOrderResponseBuilder {
    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn prescriptions(mut self, value: Vec<String>) -> Self {
        self.prescriptions = Some(value);
        self
    }

    pub fn signed_at(mut self, value: impl Into<String>) -> Self {
        self.signed_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: SignOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`order_id`](SignOrderResponseBuilder::order_id)
    /// - [`prescriptions`](SignOrderResponseBuilder::prescriptions)
    /// - [`signed_at`](SignOrderResponseBuilder::signed_at)
    /// - [`status`](SignOrderResponseBuilder::status)
    pub fn build(self) -> Result<SignOrderResponse, BuildError> {
        Ok(SignOrderResponse {
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            prescriptions: self
                .prescriptions
                .ok_or_else(|| BuildError::missing_field("prescriptions"))?,
            signed_at: self
                .signed_at
                .ok_or_else(|| BuildError::missing_field("signed_at"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

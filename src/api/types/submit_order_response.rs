pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubmitOrderResponse {
    pub object: SubmitOrderResponseObject,
    #[serde(rename = "orderId")]
    #[serde(default)]
    pub order_id: String,
    pub status: SubmitOrderResponseStatus,
}

impl SubmitOrderResponse {
    pub fn builder() -> SubmitOrderResponseBuilder {
        <SubmitOrderResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitOrderResponseBuilder {
    object: Option<SubmitOrderResponseObject>,
    order_id: Option<String>,
    status: Option<SubmitOrderResponseStatus>,
}

impl SubmitOrderResponseBuilder {
    pub fn object(mut self, value: SubmitOrderResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: SubmitOrderResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitOrderResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](SubmitOrderResponseBuilder::object)
    /// - [`order_id`](SubmitOrderResponseBuilder::order_id)
    /// - [`status`](SubmitOrderResponseBuilder::status)
    pub fn build(self) -> Result<SubmitOrderResponse, BuildError> {
        Ok(SubmitOrderResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

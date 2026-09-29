pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderRequest {
    #[serde(default)]
    pub reason: String,
}

impl CancelOrderRequest {
    pub fn builder() -> CancelOrderRequestBuilder {
        <CancelOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderRequestBuilder {
    reason: Option<String>,
}

impl CancelOrderRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](CancelOrderRequestBuilder::reason)
    pub fn build(self) -> Result<CancelOrderRequest, BuildError> {
        Ok(CancelOrderRequest {
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignAndSubmitOrderResponsePrescriptionsItemError {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub detail: String,
    pub status: SignAndSubmitOrderResponsePrescriptionsItemErrorStatus,
}

impl SignAndSubmitOrderResponsePrescriptionsItemError {
    pub fn builder() -> SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder {
        <SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder {
    code: Option<String>,
    detail: Option<String>,
    status: Option<SignAndSubmitOrderResponsePrescriptionsItemErrorStatus>,
}

impl SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn status(mut self, value: SignAndSubmitOrderResponsePrescriptionsItemErrorStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderResponsePrescriptionsItemError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder::code)
    /// - [`detail`](SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder::detail)
    /// - [`status`](SignAndSubmitOrderResponsePrescriptionsItemErrorBuilder::status)
    pub fn build(self) -> Result<SignAndSubmitOrderResponsePrescriptionsItemError, BuildError> {
        Ok(SignAndSubmitOrderResponsePrescriptionsItemError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

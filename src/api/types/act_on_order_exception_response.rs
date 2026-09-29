pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ActOnOrderExceptionResponse {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "exceptionId")]
    #[serde(default)]
    pub exception_id: String,
    pub status: ActOnOrderExceptionResponseStatus,
}

impl ActOnOrderExceptionResponse {
    pub fn builder() -> ActOnOrderExceptionResponseBuilder {
        <ActOnOrderExceptionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActOnOrderExceptionResponseBuilder {
    action: Option<String>,
    exception_id: Option<String>,
    status: Option<ActOnOrderExceptionResponseStatus>,
}

impl ActOnOrderExceptionResponseBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn exception_id(mut self, value: impl Into<String>) -> Self {
        self.exception_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ActOnOrderExceptionResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ActOnOrderExceptionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ActOnOrderExceptionResponseBuilder::action)
    /// - [`exception_id`](ActOnOrderExceptionResponseBuilder::exception_id)
    /// - [`status`](ActOnOrderExceptionResponseBuilder::status)
    pub fn build(self) -> Result<ActOnOrderExceptionResponse, BuildError> {
        Ok(ActOnOrderExceptionResponse {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            exception_id: self
                .exception_id
                .ok_or_else(|| BuildError::missing_field("exception_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

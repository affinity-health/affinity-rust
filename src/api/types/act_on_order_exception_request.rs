pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ActOnOrderExceptionRequest {
    pub action: ActOnOrderExceptionRequestAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ActOnOrderExceptionRequest {
    pub fn builder() -> ActOnOrderExceptionRequestBuilder {
        <ActOnOrderExceptionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActOnOrderExceptionRequestBuilder {
    action: Option<ActOnOrderExceptionRequestAction>,
    note: Option<String>,
}

impl ActOnOrderExceptionRequestBuilder {
    pub fn action(mut self, value: ActOnOrderExceptionRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActOnOrderExceptionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ActOnOrderExceptionRequestBuilder::action)
    pub fn build(self) -> Result<ActOnOrderExceptionRequest, BuildError> {
        Ok(ActOnOrderExceptionRequest {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            note: self.note,
        })
    }
}

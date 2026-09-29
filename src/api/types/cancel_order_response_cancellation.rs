pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CancelOrderResponseCancellation {
    pub status: CancelOrderResponseCancellationStatus,
    #[serde(default)]
    pub outcomes: Vec<CancelOrderResponseCancellationOutcomesItem>,
}

impl CancelOrderResponseCancellation {
    pub fn builder() -> CancelOrderResponseCancellationBuilder {
        <CancelOrderResponseCancellationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponseCancellationBuilder {
    status: Option<CancelOrderResponseCancellationStatus>,
    outcomes: Option<Vec<CancelOrderResponseCancellationOutcomesItem>>,
}

impl CancelOrderResponseCancellationBuilder {
    pub fn status(mut self, value: CancelOrderResponseCancellationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn outcomes(mut self, value: Vec<CancelOrderResponseCancellationOutcomesItem>) -> Self {
        self.outcomes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponseCancellation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](CancelOrderResponseCancellationBuilder::status)
    /// - [`outcomes`](CancelOrderResponseCancellationBuilder::outcomes)
    pub fn build(self) -> Result<CancelOrderResponseCancellation, BuildError> {
        Ok(CancelOrderResponseCancellation {
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            outcomes: self
                .outcomes
                .ok_or_else(|| BuildError::missing_field("outcomes"))?,
        })
    }
}

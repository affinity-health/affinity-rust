pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateOrderTestSimulationRequest {
    pub mode: UpdateOrderTestSimulationRequestMode,
    pub scenario: UpdateOrderTestSimulationRequestScenario,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<UpdateOrderTestSimulationRequestAction>,
}

impl UpdateOrderTestSimulationRequest {
    pub fn builder() -> UpdateOrderTestSimulationRequestBuilder {
        <UpdateOrderTestSimulationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderTestSimulationRequestBuilder {
    mode: Option<UpdateOrderTestSimulationRequestMode>,
    scenario: Option<UpdateOrderTestSimulationRequestScenario>,
    action: Option<UpdateOrderTestSimulationRequestAction>,
}

impl UpdateOrderTestSimulationRequestBuilder {
    pub fn mode(mut self, value: UpdateOrderTestSimulationRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn scenario(mut self, value: UpdateOrderTestSimulationRequestScenario) -> Self {
        self.scenario = Some(value);
        self
    }

    pub fn action(mut self, value: UpdateOrderTestSimulationRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderTestSimulationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](UpdateOrderTestSimulationRequestBuilder::mode)
    /// - [`scenario`](UpdateOrderTestSimulationRequestBuilder::scenario)
    pub fn build(self) -> Result<UpdateOrderTestSimulationRequest, BuildError> {
        Ok(UpdateOrderTestSimulationRequest {
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            scenario: self
                .scenario
                .ok_or_else(|| BuildError::missing_field("scenario"))?,
            action: self.action,
        })
    }
}

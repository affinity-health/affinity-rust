pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateOrderTestSimulationResponse {
    pub mode: UpdateOrderTestSimulationResponseMode,
    pub scenario: UpdateOrderTestSimulationResponseScenario,
    #[serde(rename = "pendingAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_action: Option<String>,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "availableActions")]
    #[serde(default)]
    pub available_actions: Vec<UpdateOrderTestSimulationResponseAvailableActionsItem>,
    #[serde(rename = "scenarioEditable")]
    #[serde(default)]
    pub scenario_editable: bool,
}

impl UpdateOrderTestSimulationResponse {
    pub fn builder() -> UpdateOrderTestSimulationResponseBuilder {
        <UpdateOrderTestSimulationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderTestSimulationResponseBuilder {
    mode: Option<UpdateOrderTestSimulationResponseMode>,
    scenario: Option<UpdateOrderTestSimulationResponseScenario>,
    pending_action: Option<String>,
    last_error: Option<String>,
    available_actions: Option<Vec<UpdateOrderTestSimulationResponseAvailableActionsItem>>,
    scenario_editable: Option<bool>,
}

impl UpdateOrderTestSimulationResponseBuilder {
    pub fn mode(mut self, value: UpdateOrderTestSimulationResponseMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn scenario(mut self, value: UpdateOrderTestSimulationResponseScenario) -> Self {
        self.scenario = Some(value);
        self
    }

    pub fn pending_action(mut self, value: impl Into<String>) -> Self {
        self.pending_action = Some(value.into());
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn available_actions(
        mut self,
        value: Vec<UpdateOrderTestSimulationResponseAvailableActionsItem>,
    ) -> Self {
        self.available_actions = Some(value);
        self
    }

    pub fn scenario_editable(mut self, value: bool) -> Self {
        self.scenario_editable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderTestSimulationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](UpdateOrderTestSimulationResponseBuilder::mode)
    /// - [`scenario`](UpdateOrderTestSimulationResponseBuilder::scenario)
    /// - [`available_actions`](UpdateOrderTestSimulationResponseBuilder::available_actions)
    /// - [`scenario_editable`](UpdateOrderTestSimulationResponseBuilder::scenario_editable)
    pub fn build(self) -> Result<UpdateOrderTestSimulationResponse, BuildError> {
        Ok(UpdateOrderTestSimulationResponse {
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            scenario: self
                .scenario
                .ok_or_else(|| BuildError::missing_field("scenario"))?,
            pending_action: self.pending_action,
            last_error: self.last_error,
            available_actions: self
                .available_actions
                .ok_or_else(|| BuildError::missing_field("available_actions"))?,
            scenario_editable: self
                .scenario_editable
                .ok_or_else(|| BuildError::missing_field("scenario_editable"))?,
        })
    }
}

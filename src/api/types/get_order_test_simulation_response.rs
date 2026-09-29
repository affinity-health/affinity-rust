pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetOrderTestSimulationResponse {
    pub mode: GetOrderTestSimulationResponseMode,
    pub scenario: GetOrderTestSimulationResponseScenario,
    #[serde(rename = "pendingAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_action: Option<String>,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "availableActions")]
    #[serde(default)]
    pub available_actions: Vec<GetOrderTestSimulationResponseAvailableActionsItem>,
    #[serde(rename = "scenarioEditable")]
    #[serde(default)]
    pub scenario_editable: bool,
}

impl GetOrderTestSimulationResponse {
    pub fn builder() -> GetOrderTestSimulationResponseBuilder {
        <GetOrderTestSimulationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderTestSimulationResponseBuilder {
    mode: Option<GetOrderTestSimulationResponseMode>,
    scenario: Option<GetOrderTestSimulationResponseScenario>,
    pending_action: Option<String>,
    last_error: Option<String>,
    available_actions: Option<Vec<GetOrderTestSimulationResponseAvailableActionsItem>>,
    scenario_editable: Option<bool>,
}

impl GetOrderTestSimulationResponseBuilder {
    pub fn mode(mut self, value: GetOrderTestSimulationResponseMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn scenario(mut self, value: GetOrderTestSimulationResponseScenario) -> Self {
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
        value: Vec<GetOrderTestSimulationResponseAvailableActionsItem>,
    ) -> Self {
        self.available_actions = Some(value);
        self
    }

    pub fn scenario_editable(mut self, value: bool) -> Self {
        self.scenario_editable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOrderTestSimulationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](GetOrderTestSimulationResponseBuilder::mode)
    /// - [`scenario`](GetOrderTestSimulationResponseBuilder::scenario)
    /// - [`available_actions`](GetOrderTestSimulationResponseBuilder::available_actions)
    /// - [`scenario_editable`](GetOrderTestSimulationResponseBuilder::scenario_editable)
    pub fn build(self) -> Result<GetOrderTestSimulationResponse, BuildError> {
        Ok(GetOrderTestSimulationResponse {
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

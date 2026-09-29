pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestProfileDetailsIdentifiersItem {
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub description: String,
}

impl RegisterUserRequestProfileDetailsIdentifiersItem {
    pub fn builder() -> RegisterUserRequestProfileDetailsIdentifiersItemBuilder {
        <RegisterUserRequestProfileDetailsIdentifiersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestProfileDetailsIdentifiersItemBuilder {
    identifier: Option<String>,
    issuer: Option<String>,
    state: Option<String>,
    description: Option<String>,
}

impl RegisterUserRequestProfileDetailsIdentifiersItemBuilder {
    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequestProfileDetailsIdentifiersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`identifier`](RegisterUserRequestProfileDetailsIdentifiersItemBuilder::identifier)
    /// - [`issuer`](RegisterUserRequestProfileDetailsIdentifiersItemBuilder::issuer)
    /// - [`state`](RegisterUserRequestProfileDetailsIdentifiersItemBuilder::state)
    /// - [`description`](RegisterUserRequestProfileDetailsIdentifiersItemBuilder::description)
    pub fn build(self) -> Result<RegisterUserRequestProfileDetailsIdentifiersItem, BuildError> {
        Ok(RegisterUserRequestProfileDetailsIdentifiersItem {
            identifier: self
                .identifier
                .ok_or_else(|| BuildError::missing_field("identifier"))?,
            issuer: self
                .issuer
                .ok_or_else(|| BuildError::missing_field("issuer"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}

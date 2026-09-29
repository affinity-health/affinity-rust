pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestProfileDetailsEndpointsItem {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub r#use: String,
    #[serde(default)]
    pub affiliation: String,
}

impl RegisterUserRequestProfileDetailsEndpointsItem {
    pub fn builder() -> RegisterUserRequestProfileDetailsEndpointsItemBuilder {
        <RegisterUserRequestProfileDetailsEndpointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestProfileDetailsEndpointsItemBuilder {
    endpoint: Option<String>,
    r#type: Option<String>,
    description: Option<String>,
    r#use: Option<String>,
    affiliation: Option<String>,
}

impl RegisterUserRequestProfileDetailsEndpointsItemBuilder {
    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn r#use(mut self, value: impl Into<String>) -> Self {
        self.r#use = Some(value.into());
        self
    }

    pub fn affiliation(mut self, value: impl Into<String>) -> Self {
        self.affiliation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequestProfileDetailsEndpointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint`](RegisterUserRequestProfileDetailsEndpointsItemBuilder::endpoint)
    /// - [`r#type`](RegisterUserRequestProfileDetailsEndpointsItemBuilder::r#type)
    /// - [`description`](RegisterUserRequestProfileDetailsEndpointsItemBuilder::description)
    /// - [`r#use`](RegisterUserRequestProfileDetailsEndpointsItemBuilder::r#use)
    /// - [`affiliation`](RegisterUserRequestProfileDetailsEndpointsItemBuilder::affiliation)
    pub fn build(self) -> Result<RegisterUserRequestProfileDetailsEndpointsItem, BuildError> {
        Ok(RegisterUserRequestProfileDetailsEndpointsItem {
            endpoint: self
                .endpoint
                .ok_or_else(|| BuildError::missing_field("endpoint"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            r#use: self
                .r#use
                .ok_or_else(|| BuildError::missing_field("r#use"))?,
            affiliation: self
                .affiliation
                .ok_or_else(|| BuildError::missing_field("affiliation"))?,
        })
    }
}

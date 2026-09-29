pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem {
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

impl InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem {
    pub fn builder() -> InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder {
        <InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder {
    endpoint: Option<String>,
    r#type: Option<String>,
    description: Option<String>,
    r#use: Option<String>,
    affiliation: Option<String>,
}

impl InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder {
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

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint`](InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder::endpoint)
    /// - [`r#type`](InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder::r#type)
    /// - [`description`](InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder::description)
    /// - [`r#use`](InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder::r#use)
    /// - [`affiliation`](InvitePracticeTeamPersonRequestProfileDetailsEndpointsItemBuilder::affiliation)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem, BuildError> {
        Ok(InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem {
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

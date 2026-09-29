pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub credentials: String,
    #[serde(default)]
    pub r#type: String,
}

impl InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem {
    pub fn builder() -> InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder {
        <InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder {
    name: Option<String>,
    credentials: Option<String>,
    r#type: Option<String>,
}

impl InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder::name)
    /// - [`credentials`](InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder::credentials)
    /// - [`r#type`](InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItemBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem, BuildError> {
        Ok(
            InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                credentials: self
                    .credentials
                    .ok_or_else(|| BuildError::missing_field("credentials"))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| BuildError::missing_field("r#type"))?,
            },
        )
    }
}

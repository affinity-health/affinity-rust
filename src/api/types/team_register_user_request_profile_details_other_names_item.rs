pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestProfileDetailsOtherNamesItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub credentials: String,
    #[serde(default)]
    pub r#type: String,
}

impl RegisterUserRequestProfileDetailsOtherNamesItem {
    pub fn builder() -> RegisterUserRequestProfileDetailsOtherNamesItemBuilder {
        <RegisterUserRequestProfileDetailsOtherNamesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestProfileDetailsOtherNamesItemBuilder {
    name: Option<String>,
    credentials: Option<String>,
    r#type: Option<String>,
}

impl RegisterUserRequestProfileDetailsOtherNamesItemBuilder {
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

    /// Consumes the builder and constructs a [`RegisterUserRequestProfileDetailsOtherNamesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RegisterUserRequestProfileDetailsOtherNamesItemBuilder::name)
    /// - [`credentials`](RegisterUserRequestProfileDetailsOtherNamesItemBuilder::credentials)
    /// - [`r#type`](RegisterUserRequestProfileDetailsOtherNamesItemBuilder::r#type)
    pub fn build(self) -> Result<RegisterUserRequestProfileDetailsOtherNamesItem, BuildError> {
        Ok(RegisterUserRequestProfileDetailsOtherNamesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            credentials: self
                .credentials
                .ok_or_else(|| BuildError::missing_field("credentials"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

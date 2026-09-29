pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterUserRequestProfileDetailsSpecialtiesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub primary: bool,
}

impl RegisterUserRequestProfileDetailsSpecialtiesItem {
    pub fn builder() -> RegisterUserRequestProfileDetailsSpecialtiesItemBuilder {
        <RegisterUserRequestProfileDetailsSpecialtiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterUserRequestProfileDetailsSpecialtiesItemBuilder {
    code: Option<String>,
    description: Option<String>,
    primary: Option<bool>,
}

impl RegisterUserRequestProfileDetailsSpecialtiesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn primary(mut self, value: bool) -> Self {
        self.primary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RegisterUserRequestProfileDetailsSpecialtiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RegisterUserRequestProfileDetailsSpecialtiesItemBuilder::code)
    /// - [`description`](RegisterUserRequestProfileDetailsSpecialtiesItemBuilder::description)
    /// - [`primary`](RegisterUserRequestProfileDetailsSpecialtiesItemBuilder::primary)
    pub fn build(self) -> Result<RegisterUserRequestProfileDetailsSpecialtiesItem, BuildError> {
        Ok(RegisterUserRequestProfileDetailsSpecialtiesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            primary: self
                .primary
                .ok_or_else(|| BuildError::missing_field("primary"))?,
        })
    }
}

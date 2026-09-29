pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseOrderInputPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl PreviewOrderResponseOrderInputPrescriberProfile {
    pub fn builder() -> PreviewOrderResponseOrderInputPrescriberProfileBuilder {
        <PreviewOrderResponseOrderInputPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl PreviewOrderResponseOrderInputPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriberProfile`].
    pub fn build(self) -> Result<PreviewOrderResponseOrderInputPrescriberProfile, BuildError> {
        Ok(PreviewOrderResponseOrderInputPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

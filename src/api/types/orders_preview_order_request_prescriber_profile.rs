pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl PreviewOrderRequestPrescriberProfile {
    pub fn builder() -> PreviewOrderRequestPrescriberProfileBuilder {
        <PreviewOrderRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl PreviewOrderRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriberProfile`].
    pub fn build(self) -> Result<PreviewOrderRequestPrescriberProfile, BuildError> {
        Ok(PreviewOrderRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

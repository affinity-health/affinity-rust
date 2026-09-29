pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitOrderRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl SubmitOrderRequestPrescriberProfile {
    pub fn builder() -> SubmitOrderRequestPrescriberProfileBuilder {
        <SubmitOrderRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitOrderRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl SubmitOrderRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitOrderRequestPrescriberProfile`].
    pub fn build(self) -> Result<SubmitOrderRequestPrescriberProfile, BuildError> {
        Ok(SubmitOrderRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

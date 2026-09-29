pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignOrderRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl SignOrderRequestPrescriberProfile {
    pub fn builder() -> SignOrderRequestPrescriberProfileBuilder {
        <SignOrderRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignOrderRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl SignOrderRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignOrderRequestPrescriberProfile`].
    pub fn build(self) -> Result<SignOrderRequestPrescriberProfile, BuildError> {
        Ok(SignOrderRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

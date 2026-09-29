pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignAndSubmitOrderRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl SignAndSubmitOrderRequestPrescriberProfile {
    pub fn builder() -> SignAndSubmitOrderRequestPrescriberProfileBuilder {
        <SignAndSubmitOrderRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl SignAndSubmitOrderRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderRequestPrescriberProfile`].
    pub fn build(self) -> Result<SignAndSubmitOrderRequestPrescriberProfile, BuildError> {
        Ok(SignAndSubmitOrderRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

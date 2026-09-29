pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectOrderRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl RejectOrderRequestPrescriberProfile {
    pub fn builder() -> RejectOrderRequestPrescriberProfileBuilder {
        <RejectOrderRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectOrderRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl RejectOrderRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RejectOrderRequestPrescriberProfile`].
    pub fn build(self) -> Result<RejectOrderRequestPrescriberProfile, BuildError> {
        Ok(RejectOrderRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

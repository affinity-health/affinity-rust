pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderBatchRequestPrescriberProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl CreateOrderBatchRequestPrescriberProfile {
    pub fn builder() -> CreateOrderBatchRequestPrescriberProfileBuilder {
        <CreateOrderBatchRequestPrescriberProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestPrescriberProfileBuilder {
    email: Option<String>,
    phone: Option<String>,
}

impl CreateOrderBatchRequestPrescriberProfileBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestPrescriberProfile`].
    pub fn build(self) -> Result<CreateOrderBatchRequestPrescriberProfile, BuildError> {
        Ok(CreateOrderBatchRequestPrescriberProfile {
            email: self.email,
            phone: self.phone,
        })
    }
}

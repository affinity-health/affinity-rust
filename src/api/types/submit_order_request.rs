pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitOrderRequest {
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescriber: Option<SubmitOrderRequestPrescriber>,
}

impl SubmitOrderRequest {
    pub fn builder() -> SubmitOrderRequestBuilder {
        <SubmitOrderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitOrderRequestBuilder {
    practice_id: Option<String>,
    user_id: Option<String>,
    prescriber: Option<SubmitOrderRequestPrescriber>,
}

impl SubmitOrderRequestBuilder {
    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn prescriber(mut self, value: SubmitOrderRequestPrescriber) -> Self {
        self.prescriber = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitOrderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`practice_id`](SubmitOrderRequestBuilder::practice_id)
    pub fn build(self) -> Result<SubmitOrderRequest, BuildError> {
        Ok(SubmitOrderRequest {
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            user_id: self.user_id,
            prescriber: self.prescriber,
        })
    }
}

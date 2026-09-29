pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesRequest {
    #[serde(default)]
    pub allergies: Vec<ReplacePatientAllergiesRequestAllergiesItem>,
    #[serde(rename = "reviewStatus")]
    pub review_status: ReplacePatientAllergiesRequestReviewStatus,
}

impl ReplacePatientAllergiesRequest {
    pub fn builder() -> ReplacePatientAllergiesRequestBuilder {
        <ReplacePatientAllergiesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesRequestBuilder {
    allergies: Option<Vec<ReplacePatientAllergiesRequestAllergiesItem>>,
    review_status: Option<ReplacePatientAllergiesRequestReviewStatus>,
}

impl ReplacePatientAllergiesRequestBuilder {
    pub fn allergies(mut self, value: Vec<ReplacePatientAllergiesRequestAllergiesItem>) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn review_status(mut self, value: ReplacePatientAllergiesRequestReviewStatus) -> Self {
        self.review_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergies`](ReplacePatientAllergiesRequestBuilder::allergies)
    /// - [`review_status`](ReplacePatientAllergiesRequestBuilder::review_status)
    pub fn build(self) -> Result<ReplacePatientAllergiesRequest, BuildError> {
        Ok(ReplacePatientAllergiesRequest {
            allergies: self
                .allergies
                .ok_or_else(|| BuildError::missing_field("allergies"))?,
            review_status: self
                .review_status
                .ok_or_else(|| BuildError::missing_field("review_status"))?,
        })
    }
}

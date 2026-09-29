pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GetPatientAllergiesResponse {
    #[serde(default)]
    pub allergies: Vec<GetPatientAllergiesResponseAllergiesItem>,
    #[serde(rename = "reviewStatus")]
    pub review_status: GetPatientAllergiesResponseReviewStatus,
}

impl GetPatientAllergiesResponse {
    pub fn builder() -> GetPatientAllergiesResponseBuilder {
        <GetPatientAllergiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPatientAllergiesResponseBuilder {
    allergies: Option<Vec<GetPatientAllergiesResponseAllergiesItem>>,
    review_status: Option<GetPatientAllergiesResponseReviewStatus>,
}

impl GetPatientAllergiesResponseBuilder {
    pub fn allergies(mut self, value: Vec<GetPatientAllergiesResponseAllergiesItem>) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn review_status(mut self, value: GetPatientAllergiesResponseReviewStatus) -> Self {
        self.review_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPatientAllergiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergies`](GetPatientAllergiesResponseBuilder::allergies)
    /// - [`review_status`](GetPatientAllergiesResponseBuilder::review_status)
    pub fn build(self) -> Result<GetPatientAllergiesResponse, BuildError> {
        Ok(GetPatientAllergiesResponse {
            allergies: self
                .allergies
                .ok_or_else(|| BuildError::missing_field("allergies"))?,
            review_status: self
                .review_status
                .ok_or_else(|| BuildError::missing_field("review_status"))?,
        })
    }
}

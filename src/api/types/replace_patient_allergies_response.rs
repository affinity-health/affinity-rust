pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePatientAllergiesResponse {
    #[serde(default)]
    pub allergies: Vec<ReplacePatientAllergiesResponseAllergiesItem>,
    #[serde(rename = "reviewStatus")]
    pub review_status: ReplacePatientAllergiesResponseReviewStatus,
}

impl ReplacePatientAllergiesResponse {
    pub fn builder() -> ReplacePatientAllergiesResponseBuilder {
        <ReplacePatientAllergiesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePatientAllergiesResponseBuilder {
    allergies: Option<Vec<ReplacePatientAllergiesResponseAllergiesItem>>,
    review_status: Option<ReplacePatientAllergiesResponseReviewStatus>,
}

impl ReplacePatientAllergiesResponseBuilder {
    pub fn allergies(mut self, value: Vec<ReplacePatientAllergiesResponseAllergiesItem>) -> Self {
        self.allergies = Some(value);
        self
    }

    pub fn review_status(mut self, value: ReplacePatientAllergiesResponseReviewStatus) -> Self {
        self.review_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePatientAllergiesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergies`](ReplacePatientAllergiesResponseBuilder::allergies)
    /// - [`review_status`](ReplacePatientAllergiesResponseBuilder::review_status)
    pub fn build(self) -> Result<ReplacePatientAllergiesResponse, BuildError> {
        Ok(ReplacePatientAllergiesResponse {
            allergies: self
                .allergies
                .ok_or_else(|| BuildError::missing_field("allergies"))?,
            review_status: self
                .review_status
                .ok_or_else(|| BuildError::missing_field("review_status"))?,
        })
    }
}

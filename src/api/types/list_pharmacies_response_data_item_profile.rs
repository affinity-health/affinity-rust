pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPharmaciesResponseDataItemProfile {
    #[serde(default)]
    pub description: String,
    #[serde(rename = "effectiveAt")]
    #[serde(default)]
    pub effective_at: String,
    #[serde(rename = "monthlyPrescriptionVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_prescription_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<ListPharmaciesResponseDataItemProfileRating>,
    #[serde(rename = "ratingBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_basis: Option<String>,
    #[serde(rename = "ratingReviewCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating_review_count: Option<i64>,
    #[serde(rename = "recommendedRank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_rank: Option<i64>,
}

impl ListPharmaciesResponseDataItemProfile {
    pub fn builder() -> ListPharmaciesResponseDataItemProfileBuilder {
        <ListPharmaciesResponseDataItemProfileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPharmaciesResponseDataItemProfileBuilder {
    description: Option<String>,
    effective_at: Option<String>,
    monthly_prescription_volume: Option<i64>,
    rating: Option<ListPharmaciesResponseDataItemProfileRating>,
    rating_basis: Option<String>,
    rating_review_count: Option<i64>,
    recommended_rank: Option<i64>,
}

impl ListPharmaciesResponseDataItemProfileBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn effective_at(mut self, value: impl Into<String>) -> Self {
        self.effective_at = Some(value.into());
        self
    }

    pub fn monthly_prescription_volume(mut self, value: i64) -> Self {
        self.monthly_prescription_volume = Some(value);
        self
    }

    pub fn rating(mut self, value: ListPharmaciesResponseDataItemProfileRating) -> Self {
        self.rating = Some(value);
        self
    }

    pub fn rating_basis(mut self, value: impl Into<String>) -> Self {
        self.rating_basis = Some(value.into());
        self
    }

    pub fn rating_review_count(mut self, value: i64) -> Self {
        self.rating_review_count = Some(value);
        self
    }

    pub fn recommended_rank(mut self, value: i64) -> Self {
        self.recommended_rank = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPharmaciesResponseDataItemProfile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](ListPharmaciesResponseDataItemProfileBuilder::description)
    /// - [`effective_at`](ListPharmaciesResponseDataItemProfileBuilder::effective_at)
    pub fn build(self) -> Result<ListPharmaciesResponseDataItemProfile, BuildError> {
        Ok(ListPharmaciesResponseDataItemProfile {
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            effective_at: self
                .effective_at
                .ok_or_else(|| BuildError::missing_field("effective_at"))?,
            monthly_prescription_volume: self.monthly_prescription_volume,
            rating: self.rating,
            rating_basis: self.rating_basis,
            rating_review_count: self.rating_review_count,
            recommended_rank: self.recommended_rank,
        })
    }
}

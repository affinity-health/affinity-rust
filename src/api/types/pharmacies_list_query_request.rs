pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PharmaciesListQueryRequest {
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "shipsToState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ships_to_state: Option<String>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl PharmaciesListQueryRequest {
    pub fn builder() -> PharmaciesListQueryRequestBuilder {
        <PharmaciesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PharmaciesListQueryRequestBuilder {
    ending_before: Option<String>,
    limit: Option<i64>,
    org_id: Option<String>,
    pharmacy_id: Option<String>,
    query: Option<String>,
    ships_to_state: Option<String>,
    starting_after: Option<String>,
}

impl PharmaciesListQueryRequestBuilder {
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn pharmacy_id(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn ships_to_state(mut self, value: impl Into<String>) -> Self {
        self.ships_to_state = Some(value.into());
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PharmaciesListQueryRequest`].
    pub fn build(self) -> Result<PharmaciesListQueryRequest, BuildError> {
        Ok(PharmaciesListQueryRequest {
            ending_before: self.ending_before,
            limit: self.limit,
            org_id: self.org_id,
            pharmacy_id: self.pharmacy_id,
            query: self.query,
            ships_to_state: self.ships_to_state,
            starting_after: self.starting_after,
        })
    }
}

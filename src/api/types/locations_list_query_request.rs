pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LocationsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListLocationsRequestStatus>,
}

impl LocationsListQueryRequest {
    pub fn builder() -> LocationsListQueryRequestBuilder {
        <LocationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LocationsListQueryRequestBuilder {
    limit: Option<i64>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    status: Option<ListLocationsRequestStatus>,
}

impl LocationsListQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListLocationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LocationsListQueryRequest`].
    pub fn build(self) -> Result<LocationsListQueryRequest, BuildError> {
        Ok(LocationsListQueryRequest {
            limit: self.limit,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            status: self.status,
        })
    }
}

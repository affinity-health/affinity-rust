pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatientsAddressesListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAddressesRequestStatus>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl PatientsAddressesListQueryRequest {
    pub fn builder() -> PatientsAddressesListQueryRequestBuilder {
        <PatientsAddressesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatientsAddressesListQueryRequestBuilder {
    status: Option<ListAddressesRequestStatus>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    limit: Option<i64>,
}

impl PatientsAddressesListQueryRequestBuilder {
    pub fn status(mut self, value: ListAddressesRequestStatus) -> Self {
        self.status = Some(value);
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

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatientsAddressesListQueryRequest`].
    pub fn build(self) -> Result<PatientsAddressesListQueryRequest, BuildError> {
        Ok(PatientsAddressesListQueryRequest {
            status: self.status,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            limit: self.limit,
        })
    }
}

pub use crate::prelude::*;

/// Query parameters for listPatientAddresses
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPatientAddressesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPatientAddressesRequestStatus>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListPatientAddressesQueryRequest {
    pub fn builder() -> ListPatientAddressesQueryRequestBuilder {
        <ListPatientAddressesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientAddressesQueryRequestBuilder {
    status: Option<ListPatientAddressesRequestStatus>,
    starting_after: Option<String>,
    ending_before: Option<String>,
    limit: Option<i64>,
}

impl ListPatientAddressesQueryRequestBuilder {
    pub fn status(mut self, value: ListPatientAddressesRequestStatus) -> Self {
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

    /// Consumes the builder and constructs a [`ListPatientAddressesQueryRequest`].
    pub fn build(self) -> Result<ListPatientAddressesQueryRequest, BuildError> {
        Ok(ListPatientAddressesQueryRequest {
            status: self.status,
            starting_after: self.starting_after,
            ending_before: self.ending_before,
            limit: self.limit,
        })
    }
}

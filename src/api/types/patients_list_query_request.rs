pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatientsListQueryRequest {
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalIdentitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identity_source: Option<String>,
    #[serde(rename = "externalIdentityValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identity_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<ListPatientsRequestGender>,
    #[serde(rename = "lastOrderAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_order_after: Option<String>,
    #[serde(rename = "lastOrderBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_order_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ListPatientsRequestSort>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPatientsRequestStatus>,
}

impl PatientsListQueryRequest {
    pub fn builder() -> PatientsListQueryRequestBuilder {
        <PatientsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatientsListQueryRequestBuilder {
    ending_before: Option<String>,
    external_id: Option<String>,
    external_identity_source: Option<String>,
    external_identity_value: Option<String>,
    gender: Option<ListPatientsRequestGender>,
    last_order_after: Option<String>,
    last_order_before: Option<String>,
    limit: Option<i64>,
    program: Option<String>,
    query: Option<String>,
    sort: Option<ListPatientsRequestSort>,
    starting_after: Option<String>,
    states: Option<String>,
    status: Option<ListPatientsRequestStatus>,
}

impl PatientsListQueryRequestBuilder {
    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_identity_source(mut self, value: impl Into<String>) -> Self {
        self.external_identity_source = Some(value.into());
        self
    }

    pub fn external_identity_value(mut self, value: impl Into<String>) -> Self {
        self.external_identity_value = Some(value.into());
        self
    }

    pub fn gender(mut self, value: ListPatientsRequestGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn last_order_after(mut self, value: impl Into<String>) -> Self {
        self.last_order_after = Some(value.into());
        self
    }

    pub fn last_order_before(mut self, value: impl Into<String>) -> Self {
        self.last_order_before = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn program(mut self, value: impl Into<String>) -> Self {
        self.program = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn sort(mut self, value: ListPatientsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    pub fn states(mut self, value: impl Into<String>) -> Self {
        self.states = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPatientsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatientsListQueryRequest`].
    pub fn build(self) -> Result<PatientsListQueryRequest, BuildError> {
        Ok(PatientsListQueryRequest {
            ending_before: self.ending_before,
            external_id: self.external_id,
            external_identity_source: self.external_identity_source,
            external_identity_value: self.external_identity_value,
            gender: self.gender,
            last_order_after: self.last_order_after,
            last_order_before: self.last_order_before,
            limit: self.limit,
            program: self.program,
            query: self.query,
            sort: self.sort,
            starting_after: self.starting_after,
            states: self.states,
            status: self.status,
        })
    }
}

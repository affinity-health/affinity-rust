pub use crate::prelude::*;

/// Query parameters for getAccount
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAccountQueryRequest {
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

impl GetAccountQueryRequest {
    pub fn builder() -> GetAccountQueryRequestBuilder {
        <GetAccountQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAccountQueryRequestBuilder {
    org_id: Option<String>,
}

impl GetAccountQueryRequestBuilder {
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAccountQueryRequest`].
    pub fn build(self) -> Result<GetAccountQueryRequest, BuildError> {
        Ok(GetAccountQueryRequest {
            org_id: self.org_id,
        })
    }
}

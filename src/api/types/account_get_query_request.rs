pub use crate::prelude::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountGetQueryRequest {
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
}

impl AccountGetQueryRequest {
    pub fn builder() -> AccountGetQueryRequestBuilder {
        <AccountGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountGetQueryRequestBuilder {
    org_id: Option<String>,
}

impl AccountGetQueryRequestBuilder {
    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountGetQueryRequest`].
    pub fn build(self) -> Result<AccountGetQueryRequest, BuildError> {
        Ok(AccountGetQueryRequest {
            org_id: self.org_id,
        })
    }
}

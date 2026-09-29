pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrdersListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "externalOrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    #[serde(rename = "createdAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "createdBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "patientId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<String>,
    #[serde(rename = "patientExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_external_id: Option<String>,
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ListOrdersRequestSort>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListOrdersRequestStatus>,
}

impl OrdersListQueryRequest {
    pub fn builder() -> OrdersListQueryRequestBuilder {
        <OrdersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListQueryRequestBuilder {
    query: Option<String>,
    external_order_id: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    ending_before: Option<String>,
    limit: Option<i64>,
    order_id: Option<String>,
    patient_id: Option<String>,
    patient_external_id: Option<String>,
    practice_id: Option<String>,
    sort: Option<ListOrdersRequestSort>,
    starting_after: Option<String>,
    status: Option<ListOrdersRequestStatus>,
}

impl OrdersListQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
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

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
        self
    }

    pub fn patient_id(mut self, value: impl Into<String>) -> Self {
        self.patient_id = Some(value.into());
        self
    }

    pub fn patient_external_id(mut self, value: impl Into<String>) -> Self {
        self.patient_external_id = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn sort(mut self, value: ListOrdersRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListOrdersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrdersListQueryRequest`].
    pub fn build(self) -> Result<OrdersListQueryRequest, BuildError> {
        Ok(OrdersListQueryRequest {
            query: self.query,
            external_order_id: self.external_order_id,
            created_after: self.created_after,
            created_before: self.created_before,
            ending_before: self.ending_before,
            limit: self.limit,
            order_id: self.order_id,
            patient_id: self.patient_id,
            patient_external_id: self.patient_external_id,
            practice_id: self.practice_id,
            sort: self.sort,
            starting_after: self.starting_after,
            status: self.status,
        })
    }
}

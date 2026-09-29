pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetOrderResponsePrescriptionsItemPatientSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "allergyReviewStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergy_review_status:
        Option<GetOrderResponsePrescriptionsItemPatientSnapshotAllergyReviewStatus>,
    #[serde(rename = "dateOfBirth")]
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<GetOrderResponsePrescriptionsItemPatientSnapshotGender>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    pub legal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub state: String,
}

impl GetOrderResponsePrescriptionsItemPatientSnapshot {
    pub fn builder() -> GetOrderResponsePrescriptionsItemPatientSnapshotBuilder {
        <GetOrderResponsePrescriptionsItemPatientSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemPatientSnapshotBuilder {
    address: Option<HashMap<String, serde_json::Value>>,
    allergy_review_status:
        Option<GetOrderResponsePrescriptionsItemPatientSnapshotAllergyReviewStatus>,
    date_of_birth: Option<String>,
    email: Option<String>,
    gender: Option<GetOrderResponsePrescriptionsItemPatientSnapshotGender>,
    legal_name: Option<String>,
    phone: Option<String>,
    state: Option<String>,
}

impl GetOrderResponsePrescriptionsItemPatientSnapshotBuilder {
    pub fn address(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.address = Some(value);
        self
    }

    pub fn allergy_review_status(
        mut self,
        value: GetOrderResponsePrescriptionsItemPatientSnapshotAllergyReviewStatus,
    ) -> Self {
        self.allergy_review_status = Some(value);
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn gender(mut self, value: GetOrderResponsePrescriptionsItemPatientSnapshotGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemPatientSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_of_birth`](GetOrderResponsePrescriptionsItemPatientSnapshotBuilder::date_of_birth)
    /// - [`legal_name`](GetOrderResponsePrescriptionsItemPatientSnapshotBuilder::legal_name)
    /// - [`state`](GetOrderResponsePrescriptionsItemPatientSnapshotBuilder::state)
    pub fn build(self) -> Result<GetOrderResponsePrescriptionsItemPatientSnapshot, BuildError> {
        Ok(GetOrderResponsePrescriptionsItemPatientSnapshot {
            address: self.address,
            allergy_review_status: self.allergy_review_status,
            date_of_birth: self
                .date_of_birth
                .ok_or_else(|| BuildError::missing_field("date_of_birth"))?,
            email: self.email,
            gender: self.gender,
            legal_name: self
                .legal_name
                .ok_or_else(|| BuildError::missing_field("legal_name"))?,
            phone: self.phone,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

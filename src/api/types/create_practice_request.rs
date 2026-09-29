pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePracticeRequest {
    /// Enable Live access at creation. Requires an approved platform and a Live request. Defaults to false.
    #[serde(rename = "liveEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_enabled: Option<bool>,
    #[serde(default)]
    pub address: CreatePracticeRequestAddress,
    #[serde(default)]
    pub attestations: CreatePracticeRequestAttestations,
    #[serde(rename = "complianceContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contact: Option<CreatePracticeRequestComplianceContact>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescribers: Option<Vec<CreatePracticeRequestPrescribersItem>>,
    #[serde(rename = "primaryContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<CreatePracticeRequestPrimaryContact>,
    #[serde(rename = "supportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "supportPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    /// Optional IANA timezone override. Omit to leave unchanged; null clears it. No timezone is inferred when creating a record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl CreatePracticeRequest {
    pub fn builder() -> CreatePracticeRequestBuilder {
        <CreatePracticeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePracticeRequestBuilder {
    live_enabled: Option<bool>,
    address: Option<CreatePracticeRequestAddress>,
    attestations: Option<CreatePracticeRequestAttestations>,
    compliance_contact: Option<CreatePracticeRequestComplianceContact>,
    external_id: Option<String>,
    legal_name: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    prescribers: Option<Vec<CreatePracticeRequestPrescribersItem>>,
    primary_contact: Option<CreatePracticeRequestPrimaryContact>,
    support_email: Option<String>,
    support_phone: Option<String>,
    timezone: Option<String>,
}

impl CreatePracticeRequestBuilder {
    pub fn live_enabled(mut self, value: bool) -> Self {
        self.live_enabled = Some(value);
        self
    }

    pub fn address(mut self, value: CreatePracticeRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn attestations(mut self, value: CreatePracticeRequestAttestations) -> Self {
        self.attestations = Some(value);
        self
    }

    pub fn compliance_contact(mut self, value: CreatePracticeRequestComplianceContact) -> Self {
        self.compliance_contact = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prescribers(mut self, value: Vec<CreatePracticeRequestPrescribersItem>) -> Self {
        self.prescribers = Some(value);
        self
    }

    pub fn primary_contact(mut self, value: CreatePracticeRequestPrimaryContact) -> Self {
        self.primary_contact = Some(value);
        self
    }

    pub fn support_email(mut self, value: impl Into<String>) -> Self {
        self.support_email = Some(value.into());
        self
    }

    pub fn support_phone(mut self, value: impl Into<String>) -> Self {
        self.support_phone = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePracticeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](CreatePracticeRequestBuilder::address)
    /// - [`attestations`](CreatePracticeRequestBuilder::attestations)
    /// - [`name`](CreatePracticeRequestBuilder::name)
    pub fn build(self) -> Result<CreatePracticeRequest, BuildError> {
        Ok(CreatePracticeRequest {
            live_enabled: self.live_enabled,
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            attestations: self
                .attestations
                .ok_or_else(|| BuildError::missing_field("attestations"))?,
            compliance_contact: self.compliance_contact,
            external_id: self.external_id,
            legal_name: self.legal_name,
            metadata: self.metadata,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prescribers: self.prescribers,
            primary_contact: self.primary_contact,
            support_email: self.support_email,
            support_phone: self.support_phone,
            timezone: self.timezone,
        })
    }
}

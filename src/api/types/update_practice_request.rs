pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePracticeRequest {
    /// Enable or disable Live access for an owned practice. Requires an approved platform and a Live request. Affinity Admin decisions take precedence.
    #[serde(rename = "liveEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePracticeRequestAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestations: Option<UpdatePracticeRequestAttestations>,
    #[serde(rename = "complianceContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contact: Option<UpdatePracticeRequestComplianceContact>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescribers: Option<Vec<UpdatePracticeRequestPrescribersItem>>,
    #[serde(rename = "primaryContact")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_contact: Option<UpdatePracticeRequestPrimaryContact>,
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

impl UpdatePracticeRequest {
    pub fn builder() -> UpdatePracticeRequestBuilder {
        <UpdatePracticeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeRequestBuilder {
    live_enabled: Option<bool>,
    address: Option<UpdatePracticeRequestAddress>,
    attestations: Option<UpdatePracticeRequestAttestations>,
    compliance_contact: Option<UpdatePracticeRequestComplianceContact>,
    external_id: Option<String>,
    legal_name: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    prescribers: Option<Vec<UpdatePracticeRequestPrescribersItem>>,
    primary_contact: Option<UpdatePracticeRequestPrimaryContact>,
    support_email: Option<String>,
    support_phone: Option<String>,
    timezone: Option<String>,
}

impl UpdatePracticeRequestBuilder {
    pub fn live_enabled(mut self, value: bool) -> Self {
        self.live_enabled = Some(value);
        self
    }

    pub fn address(mut self, value: UpdatePracticeRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn attestations(mut self, value: UpdatePracticeRequestAttestations) -> Self {
        self.attestations = Some(value);
        self
    }

    pub fn compliance_contact(mut self, value: UpdatePracticeRequestComplianceContact) -> Self {
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

    pub fn prescribers(mut self, value: Vec<UpdatePracticeRequestPrescribersItem>) -> Self {
        self.prescribers = Some(value);
        self
    }

    pub fn primary_contact(mut self, value: UpdatePracticeRequestPrimaryContact) -> Self {
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

    /// Consumes the builder and constructs a [`UpdatePracticeRequest`].
    pub fn build(self) -> Result<UpdatePracticeRequest, BuildError> {
        Ok(UpdatePracticeRequest {
            live_enabled: self.live_enabled,
            address: self.address,
            attestations: self.attestations,
            compliance_contact: self.compliance_contact,
            external_id: self.external_id,
            legal_name: self.legal_name,
            metadata: self.metadata,
            name: self.name,
            prescribers: self.prescribers,
            primary_contact: self.primary_contact,
            support_email: self.support_email,
            support_phone: self.support_phone,
            timezone: self.timezone,
        })
    }
}

pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePracticeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePracticeResponseAddress>,
    #[serde(default)]
    pub contacts: UpdatePracticeResponseContacts,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub name: String,
    pub object: UpdatePracticeResponseObject,
    #[serde(default)]
    pub prescribers: Vec<UpdatePracticeResponsePrescribersItem>,
    /// Whether this practice currently has Live access. False for Test practices.
    #[serde(rename = "liveEnabled")]
    #[serde(default)]
    pub live_enabled: bool,
    #[serde(rename = "supportEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    #[serde(rename = "supportPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl UpdatePracticeResponse {
    pub fn builder() -> UpdatePracticeResponseBuilder {
        <UpdatePracticeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePracticeResponseBuilder {
    address: Option<UpdatePracticeResponseAddress>,
    contacts: Option<UpdatePracticeResponseContacts>,
    created_at: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    legal_name: Option<String>,
    livemode: Option<bool>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    object: Option<UpdatePracticeResponseObject>,
    prescribers: Option<Vec<UpdatePracticeResponsePrescribersItem>>,
    live_enabled: Option<bool>,
    support_email: Option<String>,
    support_phone: Option<String>,
    timezone: Option<String>,
}

impl UpdatePracticeResponseBuilder {
    pub fn address(mut self, value: UpdatePracticeResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn contacts(mut self, value: UpdatePracticeResponseContacts) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
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

    pub fn object(mut self, value: UpdatePracticeResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn prescribers(mut self, value: Vec<UpdatePracticeResponsePrescribersItem>) -> Self {
        self.prescribers = Some(value);
        self
    }

    pub fn live_enabled(mut self, value: bool) -> Self {
        self.live_enabled = Some(value);
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

    /// Consumes the builder and constructs a [`UpdatePracticeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contacts`](UpdatePracticeResponseBuilder::contacts)
    /// - [`created_at`](UpdatePracticeResponseBuilder::created_at)
    /// - [`id`](UpdatePracticeResponseBuilder::id)
    /// - [`livemode`](UpdatePracticeResponseBuilder::livemode)
    /// - [`metadata`](UpdatePracticeResponseBuilder::metadata)
    /// - [`name`](UpdatePracticeResponseBuilder::name)
    /// - [`object`](UpdatePracticeResponseBuilder::object)
    /// - [`prescribers`](UpdatePracticeResponseBuilder::prescribers)
    /// - [`live_enabled`](UpdatePracticeResponseBuilder::live_enabled)
    pub fn build(self) -> Result<UpdatePracticeResponse, BuildError> {
        Ok(UpdatePracticeResponse {
            address: self.address,
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            external_id: self.external_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            legal_name: self.legal_name,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            prescribers: self
                .prescribers
                .ok_or_else(|| BuildError::missing_field("prescribers"))?,
            live_enabled: self
                .live_enabled
                .ok_or_else(|| BuildError::missing_field("live_enabled"))?,
            support_email: self.support_email,
            support_phone: self.support_phone,
            timezone: self.timezone,
        })
    }
}

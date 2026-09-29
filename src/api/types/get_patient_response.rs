pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPatientResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<GetPatientResponseAddress>,
    #[serde(rename = "defaultShippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_address_id: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<GetPatientResponseShippingAddress>,
    #[serde(rename = "allergyReviewStatus")]
    pub allergy_review_status: GetPatientResponseAllergyReviewStatus,
    #[serde(rename = "allergySummary")]
    #[serde(default)]
    pub allergy_summary: Vec<GetPatientResponseAllergySummaryItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "clinicalProfile")]
    #[serde(default)]
    pub clinical_profile: GetPatientResponseClinicalProfile,
    #[serde(rename = "dateOfBirth")]
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalIdentities")]
    #[serde(default)]
    pub external_identities: Vec<GetPatientResponseExternalIdentitiesItem>,
    #[serde(default)]
    pub addresses: Vec<GetPatientResponseAddressesItem>,
    #[serde(default)]
    pub encounters: Vec<GetPatientResponseEncountersItem>,
    pub gender: GetPatientResponseGender,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub location: GetPatientResponseLocation,
    #[serde(rename = "locationId")]
    #[serde(default)]
    pub location_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(default)]
    pub measurements: Vec<GetPatientResponseMeasurementsItem>,
    #[serde(default)]
    pub name: GetPatientResponseName,
    pub object: GetPatientResponseObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub programs: Vec<GetPatientResponseProgramsItem>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    pub status: GetPatientResponseStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl GetPatientResponse {
    pub fn builder() -> GetPatientResponseBuilder {
        <GetPatientResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPatientResponseBuilder {
    address: Option<GetPatientResponseAddress>,
    default_shipping_address_id: Option<String>,
    shipping_address: Option<GetPatientResponseShippingAddress>,
    allergy_review_status: Option<GetPatientResponseAllergyReviewStatus>,
    allergy_summary: Option<Vec<GetPatientResponseAllergySummaryItem>>,
    created_at: Option<String>,
    clinical_profile: Option<GetPatientResponseClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<GetPatientResponseExternalIdentitiesItem>>,
    addresses: Option<Vec<GetPatientResponseAddressesItem>>,
    encounters: Option<Vec<GetPatientResponseEncountersItem>>,
    gender: Option<GetPatientResponseGender>,
    id: Option<String>,
    livemode: Option<bool>,
    location: Option<GetPatientResponseLocation>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<GetPatientResponseMeasurementsItem>>,
    name: Option<GetPatientResponseName>,
    object: Option<GetPatientResponseObject>,
    phone: Option<String>,
    programs: Option<Vec<GetPatientResponseProgramsItem>>,
    practice_id: Option<String>,
    status: Option<GetPatientResponseStatus>,
    updated_at: Option<String>,
}

impl GetPatientResponseBuilder {
    pub fn address(mut self, value: GetPatientResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn default_shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.default_shipping_address_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: GetPatientResponseShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn allergy_review_status(mut self, value: GetPatientResponseAllergyReviewStatus) -> Self {
        self.allergy_review_status = Some(value);
        self
    }

    pub fn allergy_summary(mut self, value: Vec<GetPatientResponseAllergySummaryItem>) -> Self {
        self.allergy_summary = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn clinical_profile(mut self, value: GetPatientResponseClinicalProfile) -> Self {
        self.clinical_profile = Some(value);
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

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_identities(
        mut self,
        value: Vec<GetPatientResponseExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<GetPatientResponseAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<GetPatientResponseEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: GetPatientResponseGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn location(mut self, value: GetPatientResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn medical_record_number(mut self, value: impl Into<String>) -> Self {
        self.medical_record_number = Some(value.into());
        self
    }

    pub fn measurements(mut self, value: Vec<GetPatientResponseMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: GetPatientResponseName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn object(mut self, value: GetPatientResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<GetPatientResponseProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetPatientResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetPatientResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergy_review_status`](GetPatientResponseBuilder::allergy_review_status)
    /// - [`allergy_summary`](GetPatientResponseBuilder::allergy_summary)
    /// - [`created_at`](GetPatientResponseBuilder::created_at)
    /// - [`clinical_profile`](GetPatientResponseBuilder::clinical_profile)
    /// - [`date_of_birth`](GetPatientResponseBuilder::date_of_birth)
    /// - [`external_identities`](GetPatientResponseBuilder::external_identities)
    /// - [`addresses`](GetPatientResponseBuilder::addresses)
    /// - [`encounters`](GetPatientResponseBuilder::encounters)
    /// - [`gender`](GetPatientResponseBuilder::gender)
    /// - [`id`](GetPatientResponseBuilder::id)
    /// - [`livemode`](GetPatientResponseBuilder::livemode)
    /// - [`location`](GetPatientResponseBuilder::location)
    /// - [`location_id`](GetPatientResponseBuilder::location_id)
    /// - [`metadata`](GetPatientResponseBuilder::metadata)
    /// - [`measurements`](GetPatientResponseBuilder::measurements)
    /// - [`name`](GetPatientResponseBuilder::name)
    /// - [`object`](GetPatientResponseBuilder::object)
    /// - [`programs`](GetPatientResponseBuilder::programs)
    /// - [`practice_id`](GetPatientResponseBuilder::practice_id)
    /// - [`status`](GetPatientResponseBuilder::status)
    /// - [`updated_at`](GetPatientResponseBuilder::updated_at)
    pub fn build(self) -> Result<GetPatientResponse, BuildError> {
        Ok(GetPatientResponse {
            address: self.address,
            default_shipping_address_id: self.default_shipping_address_id,
            shipping_address: self.shipping_address,
            allergy_review_status: self
                .allergy_review_status
                .ok_or_else(|| BuildError::missing_field("allergy_review_status"))?,
            allergy_summary: self
                .allergy_summary
                .ok_or_else(|| BuildError::missing_field("allergy_summary"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            clinical_profile: self
                .clinical_profile
                .ok_or_else(|| BuildError::missing_field("clinical_profile"))?,
            date_of_birth: self
                .date_of_birth
                .ok_or_else(|| BuildError::missing_field("date_of_birth"))?,
            email: self.email,
            external_id: self.external_id,
            external_identities: self
                .external_identities
                .ok_or_else(|| BuildError::missing_field("external_identities"))?,
            addresses: self
                .addresses
                .ok_or_else(|| BuildError::missing_field("addresses"))?,
            encounters: self
                .encounters
                .ok_or_else(|| BuildError::missing_field("encounters"))?,
            gender: self
                .gender
                .ok_or_else(|| BuildError::missing_field("gender"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            location_id: self
                .location_id
                .ok_or_else(|| BuildError::missing_field("location_id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            medical_record_number: self.medical_record_number,
            measurements: self
                .measurements
                .ok_or_else(|| BuildError::missing_field("measurements"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            phone: self.phone,
            programs: self
                .programs
                .ok_or_else(|| BuildError::missing_field("programs"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
